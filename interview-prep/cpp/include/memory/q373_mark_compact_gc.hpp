// Question #373: Mark-Compact GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-compact, compaction, fragmentation, forwarding
// Description: Implement a mark-compact collector that eliminates fragmentation by sliding live objects.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Mark-Compact GC
// Question ID: 373
class MarkCompactGc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MarkCompactGc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
