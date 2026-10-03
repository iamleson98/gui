// Question #401: OOM Killer
// Category: Memory Management | Difficulty: Hard
// Concepts: OOM, killer, victim selection, memory score
// Description: Design an out-of-memory killer that selects victims based on memory score.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// OOM Killer
// Question ID: 401
class OomKiller {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit OomKiller(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
