//! Segment Tree with Lazy Propagation — range update + range query.
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut st = Self { n, tree: vec![0; 4 * n], lazy: vec![0; 4 * n] };
        if n > 0 {
            st.build(arr, 0, 0, n - 1);
        }
        st
    }

    fn build(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
            return;
        }
        let mid = (start + end) / 2;
        self.build(arr, 2 * node + 1, start, mid);
        self.build(arr, 2 * node + 2, mid + 1, end);
        self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
    }

    fn push_down(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += (end - start + 1) as i64 * self.lazy[node];
            if start != end {
                self.lazy[2 * node + 1] += self.lazy[node];
                self.lazy[2 * node + 2] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }

    pub fn update_range(&mut self, l: usize, r: usize, val: i64) {
        if self.n == 0 {
            return;
        }
        self.update_range_helper(0, 0, self.n - 1, l, r, val);
    }

    fn update_range_helper(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i64) {
        self.push_down(node, start, end);
        if start > r || end < l {
            return;
        }
        if l <= start && end <= r {
            self.lazy[node] += val;
            self.push_down(node, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update_range_helper(2 * node + 1, start, mid, l, r, val);
        self.update_range_helper(2 * node + 2, mid + 1, end, l, r, val);
        self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
    }

    pub fn query_range(&mut self, l: usize, r: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query_range_helper(0, 0, self.n - 1, l, r)
    }

    fn query_range_helper(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        self.push_down(node, start, end);
        if start > r || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        self.query_range_helper(2 * node + 1, start, mid, l, r)
            + self.query_range_helper(2 * node + 2, mid + 1, end, l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::new(&arr);
        assert_eq!(st.query_range(0, 5), 36);
        assert_eq!(st.query_range(1, 3), 15);
    }

    #[test]
    fn test_range_update() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut st = SegmentTree::new(&arr);
        st.update_range(0, 2, 10);
        assert_eq!(st.query_range(0, 2), 36);
        assert_eq!(st.query_range(3, 4), 9);
    }
}
