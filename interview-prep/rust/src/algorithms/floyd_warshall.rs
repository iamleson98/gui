//! Question #181: Floyd-Warshall for All-Pairs Shortest Paths
//! Category: Algorithms
//! Difficulty: Hard
//! Concepts: Floyd-Warshall, k-iteration, negative-cycle detection

pub const INF: i64 = 1i64 << 60;

pub fn floyd_warshall(dist: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = dist.len();
    let mut result = dist.to_vec();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if result[i][k] != INF && result[k][j] != INF {
                    if result[i][k] + result[k][j] < result[i][j] {
                        result[i][j] = result[i][k] + result[k][j];
                    }
                }
            }
        }
    }
    result
}

pub fn has_negative_cycle(dist: &[Vec<i64>]) -> bool {
    let result = floyd_warshall(dist);
    for i in 0..dist.len() {
        if result[i][i] < 0 { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let dist = vec![
            vec![0, 3, INF, 5],
            vec![2, 0, INF, 4],
            vec![INF, 1, 0, INF],
            vec![INF, INF, 2, 0],
        ];
        let result = floyd_warshall(&dist);
        assert_eq!(result[0][2], 7);
        assert_eq!(result[2][0], 3);
    }

    #[test]
    fn test_negative_cycle() {
        let dist = vec![
            vec![0, 1, INF],
            vec![INF, 0, -1],
            vec![-1, INF, 0],
        ];
        assert!(has_negative_cycle(&dist));
    }
}
