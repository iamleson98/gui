// Question #404: Compaction
// Category: Memory Management | Difficulty: Hard
// Concepts: compaction, forwarding, fragmentation, heap
// Description: Compact the heap to reduce external fragmentation via forwarding addresses.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Compaction
// Question ID: 404
class Compaction {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit Compaction(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
