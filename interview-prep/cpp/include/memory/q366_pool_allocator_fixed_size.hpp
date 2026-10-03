// Question #366: Pool Allocator (Fixed-Size)
// Category: Memory Management | Difficulty: Hard
// Concepts: pool, fixed-size, free list, O(1)
// Description: Implement a fixed-size pool allocator using a free list for constant-time alloc/free.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Pool Allocator (Fixed-Size)
// Question ID: 366
class PoolAllocatorFixedSize {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit PoolAllocatorFixedSize(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
