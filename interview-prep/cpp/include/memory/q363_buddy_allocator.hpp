// Question #363: Buddy Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: buddy, coalescing, power-of-two, split
// Description: Implement a binary buddy allocator splitting and coalescing power-of-two blocks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Buddy Allocator
// Question ID: 363
class BuddyAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit BuddyAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
