// Question #372: Mark-and-Sweep GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-and-sweep, tracing, roots, sweep
// Description: Implement a mark-and-sweep collector that traces live objects and sweeps dead ones.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Mark-and-Sweep GC
// Question ID: 372
class MarkAndSweepGc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MarkAndSweepGc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
