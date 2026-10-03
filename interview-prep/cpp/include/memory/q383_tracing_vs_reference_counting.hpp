// Question #383: Tracing vs Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: tracing, reference counting, pause, throughput
// Description: Contrast tracing and reference counting collectors on pause time and throughput.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Tracing vs Reference Counting
// Question ID: 383
class TracingVsReferenceCounting {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit TracingVsReferenceCounting(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
