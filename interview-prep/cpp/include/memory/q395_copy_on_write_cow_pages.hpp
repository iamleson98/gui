// Question #395: Copy-on-Write (CoW) Pages
// Category: Memory Management | Difficulty: Hard
// Concepts: copy-on-write, fork, sharing, page protection
// Description: Share read-only pages and copy only on write to enable cheap fork and sharing.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Copy-on-Write (CoW) Pages
// Question ID: 395
class CopyOnWriteCowPages {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit CopyOnWriteCowPages(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
