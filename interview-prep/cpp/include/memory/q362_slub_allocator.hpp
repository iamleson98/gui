// Question #362: Slub Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: SLUB, per-CPU, freelist, kernel
// Description: Explain the SLUB allocator's simpler, per-CPU design replacing the classic slab allocator.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Slub Allocator
// Question ID: 362
class SlubAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit SlubAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
