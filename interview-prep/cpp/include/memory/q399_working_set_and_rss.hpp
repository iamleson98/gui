// Question #399: Working Set and RSS
// Category: Memory Management | Difficulty: Hard
// Concepts: working set, RSS, thrashing, sizing
// Description: Estimate the working set size and resident set to size memory and detect thrashing.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Working Set and RSS
// Question ID: 399
class WorkingSetAndRss {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit WorkingSetAndRss(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
