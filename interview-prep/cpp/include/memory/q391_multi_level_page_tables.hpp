// Question #391: Multi-Level Page Tables
// Category: Memory Management | Difficulty: Hard
// Concepts: multi-level page table, sparse, compact, translation
// Description: Design multi-level page tables to compactly represent sparse virtual address spaces.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Multi-Level Page Tables
// Question ID: 391
class MultiLevelPageTables {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MultiLevelPageTables(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
