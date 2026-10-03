// Question #405: Alignment and Padding
// Category: Memory Management | Difficulty: Hard
// Concepts: alignment, padding, struct layout, ABI
// Description: Lay out structs with alignment rules to avoid misaligned access and padding waste.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Alignment and Padding
// Question ID: 405
class AlignmentAndPadding {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit AlignmentAndPadding(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
