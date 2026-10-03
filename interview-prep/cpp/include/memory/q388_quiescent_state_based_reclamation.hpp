// Question #388: Quiescent-State-Based Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: quiescent state, reclamation, lock-free, safety
// Description: Reclaim memory at quiescent states observed across threads for lock-free safety.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Quiescent-State-Based Reclamation
// Question ID: 388
class QuiescentStateBasedReclamation {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit QuiescentStateBasedReclamation(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
