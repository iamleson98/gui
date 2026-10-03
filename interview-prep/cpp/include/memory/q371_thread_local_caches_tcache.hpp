// Question #371: Thread-Local Caches (TCache)
// Category: Memory Management | Difficulty: Hard
// Concepts: TCache, thread-local, global heap, scavenge
// Description: Design thread-local allocation caches with periodic return to the global heap.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Thread-Local Caches (TCache)
// Question ID: 371
class ThreadLocalCachesTcache {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ThreadLocalCachesTcache(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
