// Question #370: mimalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: mimalloc, per-CPU, free lists, delayed reset
// Description: Explain mimalloc's per-CPU sharded free lists and delayed resets for high throughput.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// mimalloc Design
// Question ID: 370
class MimallocDesign {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MimallocDesign(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
