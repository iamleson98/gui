// Question #385: Hazard Pointers for Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: hazard pointer, reclamation, lock-free, ABA
// Description: Use hazard pointers to safely reclaim memory in lock-free data structures.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Hazard Pointers for Reclamation
// Question ID: 385
class HazardPointersForReclamation {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit HazardPointersForReclamation(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
