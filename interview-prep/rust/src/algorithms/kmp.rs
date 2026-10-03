//! KMP String Matching — O(n+m).
pub fn compute_failure_function(pattern: &[u8]) -> Vec<usize> {
    let n = pattern.len();
    if n == 0 {
        return vec![];
    }
    let mut fail = vec![0; n];
    let mut j = 0;
    let mut i = 1;
    while i < n {
        if pattern[i] == pattern[j] {
            j += 1;
            fail[i] = j;
            i += 1;
        } else if j > 0 {
            j = fail[j - 1];
        } else {
            fail[i] = 0;
            i += 1;
        }
    }
    fail
}

pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return vec![0];
    }
    if pattern.len() > text.len() {
        return vec![];
    }
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    let fail = compute_failure_function(pattern);
    let mut result = vec![];
    let mut j = 0;
    let mut i = 0;
    while i < text.len() {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
            if j == pattern.len() {
                result.push(i - j);
                j = fail[j - 1];
            }
        } else if j > 0 {
            j = fail[j - 1];
        } else {
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(kmp_search("hello world", "world"), vec![6]);
        assert_eq!(kmp_search("abababab", "ab"), vec![0, 2, 4, 6]);
        assert_eq!(kmp_search("aaaaa", "aa"), vec![0, 1, 2, 3]);
        assert_eq!(kmp_search("abcdef", "xyz"), vec![]);
        assert_eq!(kmp_search("abcdef", "abcdef"), vec![0]);
    }
}
