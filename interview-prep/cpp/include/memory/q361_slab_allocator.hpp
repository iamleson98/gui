// Question #361: Slab Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: slab, caches, fixed-size, kernel
// Description: Implement a slab allocator caching fixed-size object states for the kernel to reduce fragmentation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Slab Allocator
// Question ID: 361
class SlabAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit SlabAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
