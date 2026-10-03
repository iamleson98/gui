// Question #376: G1 GC
// Category: Memory Management | Difficulty: Hard
// Concepts: G1, regions, pause prediction, compaction
// Description: Explain the Garbage-First collector's region-based layout and pause-time predictability.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// G1 GC
// Question ID: 376
class G1Gc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit G1Gc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
