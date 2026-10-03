// Question #410: Huge Page Table Walks
// Category: Memory Management | Difficulty: Hard
// Concepts: page walk, huge page, TLB, cost
// Description: Analyze page-walk cost with huge pages and the resulting TLB savings.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Huge Page Table Walks
// Question ID: 410
class HugePageTableWalks {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit HugePageTableWalks(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
