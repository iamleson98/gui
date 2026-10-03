// Question #369: tcmalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: tcmalloc, thread-local, spans, central heap
// Description: Explain tcmalloc's thread-local caches and span-based central heap for scalable allocation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// tcmalloc Design
// Question ID: 369
class TcmallocDesign {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit TcmallocDesign(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
