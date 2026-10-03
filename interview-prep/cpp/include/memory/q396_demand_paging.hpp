// Question #396: Demand Paging
// Category: Memory Management | Difficulty: Hard
// Concepts: demand paging, page fault, lazy, zero fill
// Description: Load pages on first access via page faults to avoid eager allocation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Demand Paging
// Question ID: 396
class DemandPaging {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit DemandPaging(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
