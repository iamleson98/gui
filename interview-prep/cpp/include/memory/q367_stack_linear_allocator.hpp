// Question #367: Stack (Linear) Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: stack allocator, markers, LIFO free, linear
// Description: Implement a stack allocator with markers to roll back to a previous top.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Stack (Linear) Allocator
// Question ID: 367
class StackLinearAllocator {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit StackLinearAllocator(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
