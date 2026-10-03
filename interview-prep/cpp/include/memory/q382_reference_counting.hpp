// Question #382: Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: reference counting, cycles, weak refs, deferred
// Description: Implement reference counting with cycle detection to reclaim unreachable cycles.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Reference Counting
// Question ID: 382
class ReferenceCounting {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ReferenceCounting(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
