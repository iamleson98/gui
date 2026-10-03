// Question #402: Overcommit and OOM
// Category: Memory Management | Difficulty: Hard
// Concepts: overcommit, commit limit, OOM, accounting
// Description: Reason about memory overcommit, commit limits, and the consequences for OOM.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Overcommit and OOM
// Question ID: 402
class OvercommitAndOom {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit OvercommitAndOom(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
