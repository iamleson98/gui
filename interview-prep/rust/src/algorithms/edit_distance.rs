//! Edit Distance (Levenshtein) — O(n*m) time, O(min(n,m)) space.
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let (s1, s2) = if s1.len() > s2.len() {
        (s2, s1)
    } else {
        (s1, s2)
    };
    let n = s1.len();
    let m = s2.len();
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];
    for j in 1..=m {
        curr[0] = j;
        for i in 1..=n {
            let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            curr[i] = (prev[i] + 1).min(curr[i - 1] + 1).min(prev[i - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("a", ""), 1);
        assert_eq!(edit_distance("", "a"), 1);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("sunday", "saturday"), 3);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("abc", "abd"), 1);
        assert_eq!(edit_distance("abc", "xyz"), 3);
    }
}
