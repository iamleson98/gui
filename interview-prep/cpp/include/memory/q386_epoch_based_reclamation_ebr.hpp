// Question #386: Epoch-Based Reclamation (EBR)
// Category: Memory Management | Difficulty: Hard
// Concepts: EBR, epoch, deferred, lock-free
// Description: Defer reclamation until epochs advance past all readers for lock-free safety.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Epoch-Based Reclamation (EBR)
// Question ID: 386
class EpochBasedReclamationEbr {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit EpochBasedReclamationEbr(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
