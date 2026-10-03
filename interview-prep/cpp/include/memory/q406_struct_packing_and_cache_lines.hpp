// Question #406: Struct Packing and Cache Lines
// Category: Memory Management | Difficulty: Hard
// Concepts: packing, cache line, layout, alignment
// Description: Pack structs to fit within cache lines and trade size against access speed.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Struct Packing and Cache Lines
// Question ID: 406
class StructPackingAndCacheLines {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit StructPackingAndCacheLines(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
