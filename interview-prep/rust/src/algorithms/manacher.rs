//! Question #168: Manacher's Algorithm — Longest Palindromic Substring
//! Category: Algorithms
//! Difficulty: Hard
//! Concepts: Manacher, transformed string, symmetry, O(n)

pub fn longest_palindrome(s: &str) -> &str {
    if s.is_empty() { return ""; }
    let chars: Vec<char> = s.chars().collect();
    // Transform: ^#c1#c2#...#cn#$
    let mut t: Vec<char> = vec!['^'];
    for &c in &chars {
        t.push('#');
        t.push(c);
    }
    t.push('#');
    t.push('$');
    let n = t.len();
    let mut p = vec![0i32; n];
    let mut c = 0;
    let mut r = 0;
    let mut max_len = 0;
    let mut center = 0;
    for i in 1..n-1 {
        let mirror = 2 * c - i;
        if (r as i32) > i as i32 {
            p[i] = (r - i).min(p[mirror] as usize) as i32;
        }
        // Expand
        while i + p[i] as usize + 1 < n && i >= p[i] as usize + 1
            && t[i + p[i] as usize + 1] == t[i - p[i] as usize - 1] {
            p[i] += 1;
        }
        if i + p[i] as usize > r {
            c = i;
            r = i + p[i] as usize;
        }
        if p[i] > max_len {
            max_len = p[i];
            center = i;
        }
    }
    let start = (center - max_len as usize) / 2;
    &s[start..(start + max_len as usize)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(longest_palindrome("babad"), "bab");
        assert_eq!(longest_palindrome("cbbd"), "bb");
        assert_eq!(longest_palindrome("a"), "a");
        assert_eq!(longest_palindrome("racecar"), "racecar");
        assert_eq!(longest_palindrome(""), "");
    }
}
