// Question #378: Shenandoah GC
// Category: Memory Management | Difficulty: Hard
// Concepts: Shenandoah, Brooks pointer, concurrent evacuation, low latency
// Description: Explain Shenandoah's concurrent evacuation using Brooks forwarding pointers.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Shenandoah GC
// Question ID: 378
class ShenandoahGc {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ShenandoahGc(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
