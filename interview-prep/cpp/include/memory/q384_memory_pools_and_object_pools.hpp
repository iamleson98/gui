// Question #384: Memory Pools and Object Pools
// Category: Memory Management | Difficulty: Hard
// Concepts: object pool, amortize, construct cost, reuse
// Description: Design object pools to amortize allocation of expensive-to-construct objects.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Memory Pools and Object Pools
// Question ID: 384
class MemoryPoolsAndObjectPools {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MemoryPoolsAndObjectPools(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
