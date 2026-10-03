// Question #389: Deferred Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: deferred RC, batching, amortize, hot path
// Description: Batch reference-count updates to amortize their cost on the hot path.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Deferred Reference Counting
// Question ID: 389
class DeferredReferenceCounting {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit DeferredReferenceCounting(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
