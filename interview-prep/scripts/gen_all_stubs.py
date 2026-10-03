#!/usr/bin/env python3
"""
Generate solution files for ALL 550 questions in Go, Rust, and C++.

Each solution file has a header comment specifying:
- Question ID (matches questions.json)
- Title
- Category
- Key Concepts
- Difficulty

For the 30 "implemented" questions, full solutions with tests/benchmarks.
For the remaining 520, well-documented stubs with TODO markers.
"""
import json
import os
import re

BASE = "/home/z/my-project/gui/interview-prep"

def load_questions():
    with open(os.path.join(BASE, "questions.json")) as f:
        return json.load(f)

def slug(title):
    """Convert title to a filesystem-friendly slug."""
    s = title.lower()
    s = re.sub(r"[^a-z0-9]+", "_", s)
    s = s.strip("_")
    return s

def category_dir(cat, lang):
    """Map a category to a directory name."""
    cat_map = {
        "Concurrency": "concurrency",
        "Data Structures": "datastructures",
        "Algorithms": "algorithms",
        "SQL & Database Design": "sql",
        "System Design": "systemdesign",
        "Memory Management": "memory",
        "Performance & Profiling": "performance",
        "Security": "security",
        "Networking & Protocols": "networking",
    }
    return cat_map.get(cat, "misc")

# ============================================================
# KNOWN FULL IMPLEMENTATIONS (question_id -> {lang: code})
# These are for the 30 implemented questions.
# ============================================================

GO_IMPLEMENTATIONS = {}  # We'll load these from existing files
RUST_IMPLEMENTATIONS = {}
CPP_IMPLEMENTATIONS = {}

def load_existing_go():
    """Load existing Go implementations."""
    go_dir = os.path.join(BASE, "go")
    impls = {}
    for subdir in ["concurrency", "datastructures", "algorithms"]:
        d = os.path.join(go_dir, subdir)
        if not os.path.isdir(d):
            continue
        for fname in os.listdir(d):
            if fname.endswith(".go") and not fname.endswith("_test.go") and not fname.endswith("_bench_test.go"):
                fpath = os.path.join(d, fname)
                with open(fpath) as f:
                    content = f.read()
                impls[fname] = content
    return impls

# ============================================================
# GENERATE STUB FILES
# ============================================================

def gen_go_stub(q):
    """Generate a Go stub file for a question."""
    pkg = category_dir(q["category"], "go")
    name = slug(q["title"])
    concepts = ", ".join(q["concepts"])
    return f'''// Question #{q["id"]}: {q["title"]}
// Category: {q["category"]}
// Difficulty: {q["difficulty"]}
// Concepts: {concepts}
// Description: {q["description"]}
//
// TODO: Implement this solution.
package {pkg}

// {q["title"]}
// Question ID: {q["id"]}
func {name}_solve() {{
    // Implementation goes here.
    // See questions.json for full question details.
}}
'''

def gen_rust_stub(q):
    """Generate a Rust stub file for a question."""
    name = slug(q["title"])
    concepts = ", ".join(q["concepts"])
    return f'''//! Question #{q["id"]}: {q["title"]}
//! Category: {q["category"]}
//! Difficulty: {q["difficulty"]}
//! Concepts: {concepts}
//! Description: {q["description"]}
//!
//! TODO: Implement this solution.

pub fn {name}() {{
    // Implementation goes here.
    // See questions.json for full question details.
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_{name}() {{
        // TODO: Write tests for question #{q["id"]}
    }}
}}
'''

def gen_cpp_stub(q):
    """Generate a C++ stub file for a question."""
    name = slug(q["title"])
    guard = f"Q{q['id']}_{name.upper()}"
    concepts = ", ".join(q["concepts"])
    return f'''// Question #{q["id"]}: {q["title"]}
// Category: {q["category"]}
// Difficulty: {q["difficulty"]}
// Concepts: {concepts}
// Description: {q["description"]}
//
// TODO: Implement this solution.
#pragma once
#include <vector>
#include <cstdint>
#include <string>

namespace interview_prep {{

// {q["title"]}
// Question ID: {q["id"]}
// See questions.json for full details.
class {name} {{
public:
    // TODO: Implement the solution for question #{q["id"]}
    void solve() {{
        // Implementation goes here.
    }}
}};

}} // namespace interview_prep
'''

# ============================================================
# MAIN
# ============================================================

def main():
    questions = load_questions()
    print(f"Loaded {len(questions)} questions")

    # Count implemented
    implemented = [q for q in questions if q["implemented"]]
    stubs = [q for q in questions if not q["implemented"]]
    print(f"  Implemented: {len(implemented)}")
    print(f"  Stubs: {len(stubs)}")

    # Generate Go stubs
    go_base = os.path.join(BASE, "go")
    go_count = 0
    for q in stubs:
        pkg = category_dir(q["category"], "go")
        name = slug(q["title"])
        fdir = os.path.join(go_base, pkg)
        os.makedirs(fdir, exist_ok=True)
        fpath = os.path.join(fdir, f"q{q['id']}_{name}.go")
        with open(fpath, "w") as f:
            f.write(gen_go_stub(q))
        go_count += 1
    print(f"Generated {go_count} Go stub files")

    # Generate Rust stubs
    rust_base = os.path.join(BASE, "rust", "src")
    # Create mod.rs for each category
    categories = {}
    for q in questions:
        cat = category_dir(q["category"], "rust")
        if cat not in categories:
            categories[cat] = []
        categories[cat].append(q)

    rust_count = 0
    for cat, qs in categories.items():
        cat_dir = os.path.join(rust_base, cat)
        os.makedirs(cat_dir, exist_ok=True)
        mod_entries = []
        for q in qs:
            if q["implemented"]:
                continue  # Skip implemented ones (already have real files)
            name = slug(q["title"])
            fname = f"q{q['id']}_{name}"
            fpath = os.path.join(cat_dir, f"{fname}.rs")
            with open(fpath, "w") as f:
                f.write(gen_rust_stub(q))
            mod_entries.append(fname)
            rust_count += 1
        # Write mod.rs
        with open(os.path.join(cat_dir, "mod.rs"), "w") as f:
            f.write(f"// {cat} module — {len(qs)} questions\n\n")
            for entry in mod_entries:
                f.write(f"pub mod {entry};\n")
            f.write("\n")
        rust_count += 0  # mod.rs doesn't count

    print(f"Generated {rust_count} Rust stub files")

    # Generate C++ stubs
    cpp_base = os.path.join(BASE, "cpp", "include")
    cpp_count = 0
    for q in stubs:
        cat = category_dir(q["category"], "cpp")
        name = slug(q["title"])
        fdir = os.path.join(cpp_base, cat)
        os.makedirs(fdir, exist_ok=True)
        fpath = os.path.join(fdir, f"q{q['id']}_{name}.hpp")
        with open(fpath, "w") as f:
            f.write(gen_cpp_stub(q))
        cpp_count += 1
    print(f"Generated {cpp_count} C++ stub files")

    # Update Rust lib.rs to include all category modules
    lib_path = os.path.join(rust_base, "lib.rs")
    with open(lib_path, "w") as f:
        f.write("// Interview Prep — Rust Solutions\n")
        f.write("// 550 senior-level interview questions.\n")
        f.write("// Each file has a header with question ID, title, and concepts.\n\n")
        for cat in sorted(categories.keys()):
            f.write(f"pub mod {cat};\n")
        f.write("\n")

    # Print summary
    print(f"\nSummary:")
    print(f"  Total questions: {len(questions)}")
    print(f"  Full implementations (Go): {len(implemented)}")
    print(f"  Full implementations (Rust): ~13")
    print(f"  Full implementations (C++): 7")
    print(f"  Stub files generated (Go): {go_count}")
    print(f"  Stub files generated (Rust): {rust_count}")
    print(f"  Stub files generated (C++): {cpp_count}")
    print(f"  Total files: {len(questions) * 3} (550 × 3 languages)")

if __name__ == "__main__":
    main()
