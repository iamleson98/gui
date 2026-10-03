// Question #374: Copying GC (Semispace)
// Category: Memory Management | Difficulty: Hard
// Concepts: copying GC, semispace, evacuation, forwarding
// Description: Implement a copying collector that evacuates live objects between two semispaces.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Copying GC (Semispace)
// Question ID: 374
class CopyingGcSemispace {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit CopyingGcSemispace(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
