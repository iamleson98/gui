// Question #390: Virtual Memory and Paging
// Category: Memory Management | Difficulty: Hard
// Concepts: virtual memory, paging, page table, translation
// Description: Implement paging that maps virtual to physical pages via page tables.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Virtual Memory and Paging
// Question ID: 390
class VirtualMemoryAndPaging {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit VirtualMemoryAndPaging(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
