// Question #381: Write Barriers (SATB/INC)
// Category: Memory Management | Difficulty: Hard
// Concepts: write barrier, SATB, incremental, invariant
// Description: Implement SATB and incremental-update write barriers to maintain tri-color invariance.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Write Barriers (SATB/INC)
// Question ID: 381
class WriteBarriersSatbInc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit WriteBarriersSatbInc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
