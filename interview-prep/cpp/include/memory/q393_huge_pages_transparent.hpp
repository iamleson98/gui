// Question #393: Huge Pages (Transparent)
// Category: Memory Management | Difficulty: Hard
// Concepts: huge pages, THP, TLB, page walk
// Description: Use huge pages to reduce TLB pressure and page-walk cost for large allocations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Huge Pages (Transparent)
// Question ID: 393
class HugePagesTransparent {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit HugePagesTransparent(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
