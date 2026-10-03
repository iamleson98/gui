#!/usr/bin/env python3
"""Fix Go files with compilation errors - targeted approach.
Only fix specific patterns:
1. Type names starting with digits (func/type X where X starts with digit)
2. interface{} -> any
3. Unbalanced braces in struct literals
"""
import os, re, glob

GO_DIR = "/home/z/my-project/gui/interview-prep/go"

def fix_go_file(filepath):
    with open(filepath) as f:
        content = f.read()
    orig = content
    
    # Fix 1: interface{} -> any (Go 1.18+)
    content = content.replace('interface{}', 'any')
    # Also fix interface{ } with space
    content = re.sub(r'interface\{\s*\}', 'any', content)
    
    # Fix 2: Type/struct names starting with digits
    # Match: type \d... or func \d... or func (...) \d...
    # Only replace in these specific contexts, not in array sizes or literals
    
    # Pattern: "type 0RttTls struct" -> "type Q_0RttTls struct"
    # Pattern: "func New0RttTls()" -> "func NewQ_0RttTls()"
    # Pattern: "func (x *0RttTls)" -> "func (x *Q_0RttTls)"
    
    def fix_identifier(match):
        prefix = match.group(1)  # e.g., "type ", "func New", "func (x *"
        ident = match.group(2)   # e.g., "0RttTls"
        suffix = match.group(3)  # e.g., " struct", "()", ")"
        return f"{prefix}Q_{ident}{suffix}"
    
    # Fix type declarations: type DigitName struct
    content = re.sub(r'(type )(\d[A-Za-z0-9_]*)(\s+struct)', fix_identifier, content)
    # Fix func declarations: func NewDigitName or func (x *DigitName)
    content = re.sub(r'(func New)(\d[A-Za-z0-9_]*)(\()', fix_identifier, content)
    content = re.sub(r'(func \(x \*)(\d[A-Za-z0-9_]*)(\))', fix_identifier, content)
    content = re.sub(r'(\*?)(\d[A-Za-z0-9_]*)(\))', lambda m: f'{m.group(1)}Q_{m.group(2)}{m.group(3)}' if m.group(2)[0].isdigit() else m.group(0), content)
    # Fix struct literal: &DigitName{ -> &Q_DigitName{
    content = re.sub(r'(&)(\d[A-Za-z0-9_]*)(\{)', lambda m: f'{m.group(1)}Q_{m.group(2)}{m.group(3)}' if m.group(2)[0].isdigit() else m.group(0), content)
    # Fix type reference: *DigitName or DigitName)
    content = re.sub(r'(\*)(\d[A-Za-z0-9_]*)', lambda m: f'{m.group(1)}Q_{m.group(2)}' if m.group(2)[0].isdigit() else m.group(0), content)
    # Fix return type: ) *DigitName -> ) *Q_DigitName
    content = re.sub(r'(\)\s*\*)(\d[A-Za-z0-9_]*)', lambda m: f'{m.group(1)}Q_{m.group(2)}' if m.group(2)[0].isdigit() else m.group(0), content)
    
    # Fix 3: map[uint64]any{} -> map[uint64]any (remove trailing {})
    content = content.replace('map[uint64]any{}', 'map[uint64]any')
    
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

if __name__ == "__main__":
    fix_all()
