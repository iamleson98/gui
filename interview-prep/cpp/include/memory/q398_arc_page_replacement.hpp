// Question #398: ARC Page Replacement
// Category: Memory Management | Difficulty: Hard
// Concepts: ARC, adaptive, recency, frequency
// Description: Implement the adaptive replacement cache policy balancing recency and frequency.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ARC Page Replacement
// Question ID: 398
class ArcPageReplacement {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ArcPageReplacement(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
