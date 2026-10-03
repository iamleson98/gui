// Question #89: Segment Tree with Lazy Propagation
// Category: Data Structures | Difficulty: Hard | Concepts: lazy propagation, range update/query
#pragma once
#include <vector>
#include <cstdint>

namespace interview_prep {

class SegmentTree {
    size_t n_;
    std::vector<int64_t> tree_, lazy_;

    void build(const std::vector<int64_t>& arr, size_t node, size_t s, size_t e) {
        if (s == e) { tree_[node] = arr[s]; return; }
        size_t mid = (s + e) / 2;
        build(arr, 2*node+1, s, mid);
        build(arr, 2*node+2, mid+1, e);
        tree_[node] = tree_[2*node+1] + tree_[2*node+2];
    }

    void push_down(size_t node, size_t s, size_t e) {
        if (lazy_[node] != 0) {
            tree_[node] += (int64_t)(e - s + 1) * lazy_[node];
            if (s != e) {
                lazy_[2*node+1] += lazy_[node];
                lazy_[2*node+2] += lazy_[node];
            }
            lazy_[node] = 0;
        }
    }

public:
    SegmentTree(const std::vector<int64_t>& arr) : n_(arr.size()), tree_(4*arr.size(), 0), lazy_(4*arr.size(), 0) {
        if (n_ > 0) build(arr, 0, 0, n_-1);
    }

    void update_range(size_t l, size_t r, int64_t val) {
        if (n_ == 0) return;
        update_range(0, 0, n_-1, l, r, val);
    }

    void update_range(size_t node, size_t s, size_t e, size_t l, size_t r, int64_t val) {
        push_down(node, s, e);
        if (s > r || e < l) return;
        if (l <= s && e <= r) { lazy_[node] += val; push_down(node, s, e); return; }
        size_t mid = (s + e) / 2;
        update_range(2*node+1, s, mid, l, r, val);
        update_range(2*node+2, mid+1, e, l, r, val);
        tree_[node] = tree_[2*node+1] + tree_[2*node+2];
    }

    int64_t query_range(size_t l, size_t r) {
        if (n_ == 0) return 0;
        return query_range(0, 0, n_-1, l, r);
    }

    int64_t query_range(size_t node, size_t s, size_t e, size_t l, size_t r) {
        push_down(node, s, e);
        if (s > r || e < l) return 0;
        if (l <= s && e <= r) return tree_[node];
        size_t mid = (s + e) / 2;
        return query_range(2*node+1, s, mid, l, r) + query_range(2*node+2, mid+1, e, l, r);
    }
};

} // namespace interview_prep
