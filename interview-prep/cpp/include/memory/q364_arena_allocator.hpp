// Question #364: Arena Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: arena, bulk alloc, reset, region
// Description: Build an arena allocator that bulk-allocates from a parent and frees all at once.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Arena Allocator
// Question ID: 364
class ArenaAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ArenaAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
