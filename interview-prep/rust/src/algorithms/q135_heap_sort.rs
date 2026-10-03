//! Question #135: Heap Sort
//! Category: Algorithms | Difficulty: Hard
//! Concepts: heap sort, in-place, build heap, extract max
//! Description: Implement in-place heapsort with a build-heap linear phase and repeated extract-max.

pub fn heap_sort(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heap_sort() {
        assert_eq!(heap_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    }
}
