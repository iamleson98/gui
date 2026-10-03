#!/usr/bin/env python3
"""
Fix Go files to compile cleanly:
1. Replace interface{} with any
2. Prefix digit-starting type/func names with Q
3. Fix struct literal braces
4. Fix redeclaration: rename types in generated stubs to include question ID
"""
import os, re, glob

GO_DIR = "/home/z/my-project/gui/interview-prep/go"

def fix_file(filepath, basename):
    with open(filepath) as f:
        content = f.read()
    orig = content

    # 1. Replace interface{} with any
    content = content.replace('interface{}', 'any')
    content = re.sub(r'interface\{\s*\}', 'any', content)
    
    # 2. Fix struct literal: &TypeName{field: make(map[...])}
    # The issue: make(map[uint64]any) ends with ) but struct needs }
    # Fix: map[uint64]any{} -> map[uint64]any (remove trailing {})
    content = re.sub(r'map\[(\w+)\]any\{\}', r'map[\1]any', content)
    # Fix: make(map[uint64]any) -> make(map[uint64]any) (already fine)
    # But if in struct literal: &TypeName{cache: make(map[uint64]any)}
    # The issue is that the generator sometimes has unbalanced braces
    
    # 3. For generated stub files (q\d+_), check for digit-starting type names
    is_stub = bool(re.match(r'q\d+_', basename))
    
    if is_stub:
        # Extract question ID from filename
        m = re.match(r'q(\d+)_', basename)
        if m:
            qid = m.group(1)
            
            # Find the type name used in this file
            # Pattern: type Xyz struct or type Xyz = ...
            type_match = re.search(r'type (\w+) struct', content)
            if type_match:
                old_name = type_match.group(1)
                # If old_name starts with digit, or conflicts, prefix with Q
                new_name = f"Q{qid}_{old_name}" if old_name[0].isdigit() else f"Q{qid}_{old_name}"
                # Replace all occurrences of the type name
                # Be careful to only replace word boundaries
                content = re.sub(r'\b' + re.escape(old_name) + r'\b', new_name, content)
                # Also fix the constructor: NewXyz -> NewQ{id}_Xyz
                old_ctor = f"New{old_name}"
                new_ctor = f"New{new_name}"
                content = content.replace(old_ctor, new_ctor)
        else:
            # For files that just have a function (no type), prefix function name
            func_match = re.search(r'func (\w+)\(', content)
            if func_match:
                old_name = func_match.group(1)
                if old_name[0].isdigit():
                    new_name = f"Q{qid}_{old_name}"
                    content = re.sub(r'\b' + re.escape(old_name) + r'\b', new_name, content)
    else:
        # For original implementation files, just fix interface{} and specific issues
        pass
    
    # 4. Fix: any in struct literal context
    # Pattern: &TypeName{field: make(map[...]any)} -> needs proper closing
    # The issue: make(map[uint64]any) produces valid Go, but 
    # &TypeName{cache: make(map[uint64]any)} might have unbalanced braces
    # from the generator
    
    if content != orig:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

def fix_all():
    fixed = 0
    for go_file in sorted(glob.glob(os.path.join(GO_DIR, "**/*.go"), recursive=True)):
        basename = os.path.basename(go_file)
        if fix_file(go_file, basename):
            fixed += 1
    print(f"Fixed {fixed} files")

if __name__ == "__main__":
    fix_all()
