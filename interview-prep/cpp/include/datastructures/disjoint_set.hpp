#pragma once
#include <vector>
#include <cstddef>

namespace interview_prep {

// Disjoint Set (Union-Find) with path compression + union by rank.
class DisjointSet {
    std::vector<size_t> parent_;
    std::vector<size_t> rank_;
public:
    explicit DisjointSet(size_t n) : parent_(n), rank_(n, 0) {
        for (size_t i = 0; i < n; ++i) parent_[i] = i;
    }

    size_t find(size_t x) {
        if (parent_[x] != x)
            parent_[x] = find(parent_[x]);
        return parent_[x];
    }

    bool union_sets(size_t a, size_t b) {
        size_t ra = find(a);
        size_t rb = find(b);
        if (ra == rb) return false;
        if (rank_[ra] < rank_[rb]) std::swap(ra, rb);
        parent_[rb] = ra;
        if (rank_[ra] == rank_[rb]) rank_[ra]++;
        return true;
    }

    bool connected(size_t a, size_t b) {
        return find(a) == find(b);
    }
};

} // namespace interview_prep
