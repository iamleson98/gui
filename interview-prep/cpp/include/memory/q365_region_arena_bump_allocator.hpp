// Question #365: Region/Arena Bump Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: bump allocator, region, O(1), bulk free
// Description: Implement a bump pointer allocator within a region for O(1) allocation and bulk free.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Region/Arena Bump Allocator
// Question ID: 365
class RegionArenaBumpAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit RegionArenaBumpAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
