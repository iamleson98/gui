// Question #379: Concurrent Marking
// Category: Memory Management | Difficulty: Hard
// Concepts: concurrent marking, safe-point, handshake, pause
// Description: Implement concurrent marking with safe-points and handshakes to avoid long pauses.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Concurrent Marking
// Question ID: 379
class ConcurrentMarking {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit ConcurrentMarking(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
