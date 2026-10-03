// Question #377: ZGC (Low Latency)
// Category: Memory Management | Difficulty: Hard
// Concepts: ZGC, colored pointer, load barrier, low latency
// Description: Explain ZGC's colored pointers and load barriers for sub-millisecond pauses.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ZGC (Low Latency)
// Question ID: 377
class ZgcLowLatency {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ZgcLowLatency(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
