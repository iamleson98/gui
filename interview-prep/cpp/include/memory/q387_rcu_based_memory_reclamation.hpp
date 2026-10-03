// Question #387: RCU-Based Memory Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: RCU, grace period, reclamation, readers
// Description: Reclaim nodes after a grace period so concurrent readers see consistent state.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// RCU-Based Memory Reclamation
// Question ID: 387
class RcuBasedMemoryReclamation {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit RcuBasedMemoryReclamation(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
