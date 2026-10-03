// Question #397: Page Replacement (LRU/Clock)
// Category: Memory Management | Difficulty: Hard
// Concepts: page replacement, LRU, clock, eviction
// Description: Implement LRU and clock page replacement policies for finite physical memory.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Page Replacement (LRU/Clock)
// Question ID: 397
class PageReplacementLruClock {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit PageReplacementLruClock(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
