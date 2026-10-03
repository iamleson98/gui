// Question #408: Cache-Friendly Data Layout (SoA vs AoS)
// Category: Memory Management | Difficulty: Hard
// Concepts: SoA, AoS, SIMD, cache
// Description: Choose between array-of-structs and struct-of-arrays for SIMD and cache efficiency.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Cache-Friendly Data Layout (SoA vs AoS)
// Question ID: 408
class CacheFriendlyDataLayoutSoaVsAos {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit CacheFriendlyDataLayoutSoaVsAos(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
