// Question #380: Tri-Color Marking
// Category: Memory Management | Difficulty: Hard
// Concepts: tri-color, white/gray/black, invariant, tracing
// Description: Implement the tri-color invariant (white, gray, black) during tracing.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Tri-Color Marking
// Question ID: 380
class TriColorMarking {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit TriColorMarking(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
