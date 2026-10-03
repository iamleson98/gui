// Question #392: TLB and TLB Shootdown
// Category: Memory Management | Difficulty: Hard
// Concepts: TLB, shootdown, IPI, translation cache
// Description: Explain the TLB cache of translations and the cost of cross-CPU shootdowns.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// TLB and TLB Shootdown
// Question ID: 392
class TlbAndTlbShootdown {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit TlbAndTlbShootdown(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
