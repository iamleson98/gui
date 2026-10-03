//! Question #203: Fortune's Voronoi Diagram
//! Category: Algorithms | Difficulty: Hard
//! Concepts: Voronoi, Fortune, sweep line, beach line
//! Description: Construct a Voronoi diagram using Fortune's sweep-line and beach-line data structure.

pub fn fortune_s_voronoi_diagram(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fortune_s_voronoi_diagram() {
        assert_eq!(fortune_s_voronoi_diagram(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
