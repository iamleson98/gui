// Question #407: False Sharing
// Category: Memory Management | Difficulty: Hard
// Concepts: false sharing, cache line, padding, contention
// Description: Diagnose false sharing on shared mutable fields in the same cache line and pad them apart.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// False Sharing
// Question ID: 407
class FalseSharing {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit FalseSharing(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
