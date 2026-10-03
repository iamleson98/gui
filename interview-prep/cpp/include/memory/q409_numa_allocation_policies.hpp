// Question #409: NUMA Allocation Policies
// Category: Memory Management | Difficulty: Hard
// Concepts: NUMA, first-touch, locality, allocation
// Description: Apply NUMA-aware allocation and first-touch to keep memory local to compute.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// NUMA Allocation Policies
// Question ID: 409
class NumaAllocationPolicies {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit NumaAllocationPolicies(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
