#!/usr/bin/env python3
"""Fix all Go files with compilation errors:
1. Identifiers starting with digits -> prefix with Q
2. interface{} type literals -> use any
3. Unbalanced braces in struct literals
"""
import os, re, glob

GO_DIR = "/home/z/my-project/gui/interview-prep/go"

def fix_go_file(filepath):
    with open(filepath) as f:
        content = f.read()
    
    orig = content
    
    # Fix: interface{} -> any (Go 1.18+)
    # But only in type contexts, not in struct literals
    content = content.replace('interface{}', 'any')
    
    # Fix: struct literals with unbalanced braces
    # Pattern: &TypeName{field: make(...)  -> needs closing }
    # The issue is make(map[...]any) inside struct literal
    content = re.sub(r'(\w+)\{(\w+):\s*make\(map\[[^\]]+\]any\)\}', r'\1{\2: make(map[string]any)}', content)
    
    # Fix: identifiers starting with digits
    # Find all type/function names that start with a digit
    lines = content.split('\n')
    fixed_lines = []
    for line in lines:
    # Replace type names starting with digit
        line = re.sub(r'\b(\d[A-Za-z0-9]*)', r'Q_\1', line)
        fixed_lines.append(line)
    content = '\n'.join(fixed_lines)
    
    # Fix: struct literal missing closing brace
    # Pattern: return &TypeName{field: value  (missing })
    content = re.sub(r'(return\s+&\w+\{[^}]+)\n\}', r'\1}\n}', content)
    
    # Fix: map[uint64]any{} -> map[uint64]any
    content = content.replace('map[uint64]any{}', 'map[uint64]any')
    content = content.replace('map[uint64]interface{}{}', 'map[uint64]any')
    
    # Fix: struct literal closing - ensure each return &X{...} has matching }
    # This is tricky - let me just ensure make(map...) in struct literals has closing }
    content = re.sub(r'(\w+):\s*make\((map\[[^\]]+\](?:any|int|string))\)\}', r'\1: make(\2)}', content)
    
    if content != orig:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

def fix_all():
    fixed = 0
    for go_file in glob.glob(os.path.join(GO_DIR, "**/*.go"), recursive=True):
        if fix_go_file(go_file):
            fixed += 1
    print(f"Fixed {fixed} files")
    return fixed

if __name__ == "__main__":
    fix_all()
