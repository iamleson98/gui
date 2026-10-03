// Question #375: Generational GC
// Category: Memory Management | Difficulty: Hard
// Concepts: generational, young/old, remembered set, promotion
// Description: Design a generational collector with young and old generations using remembered sets.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Generational GC
// Question ID: 375
class GenerationalGc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit GenerationalGc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
