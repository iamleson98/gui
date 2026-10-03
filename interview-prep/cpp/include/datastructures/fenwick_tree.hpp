// Question #90: Fenwick Tree (Binary Indexed Tree)
// Category: Data Structures | Difficulty: Hard | Concepts: BIT, LSB, prefix sum
#pragma once
#include <vector>
#include <cstdint>

namespace interview_prep {

class FenwickTree {
    std::vector<int64_t> tree_;
    size_t n_;

public:
    FenwickTree(size_t n) : tree_(n+1, 0), n_(n) {}

    FenwickTree(const std::vector<int64_t>& arr) : tree_(arr.size()+1, 0), n_(arr.size()) {
        for (size_t i = 0; i < n_; ++i) tree_[i+1] = arr[i];
        for (size_t i = 1; i <= n_; ++i) {
            size_t p = i + (i & ~(i-1));
            if (p <= n_) tree_[p] += tree_[i];
        }
    }

    void update(size_t i, int64_t delta) {
        for (i++; i <= n_; i += i & ~(i-1))
            tree_[i] += delta;
    }

    int64_t query(int64_t i) {
        int64_t sum = 0;
        for (i++; i > 0; i -= i & ~(i-1))
            sum += tree_[i];
        return sum;
    }

    int64_t query_range(size_t l, size_t r) {
        if (l > r) return 0;
        return query(r) - (l == 0 ? 0 : query(l - 1));
    }
};

} // namespace interview_prep
