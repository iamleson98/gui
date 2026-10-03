// Question #403: Memory Fragmentation (External/Internal)
// Category: Memory Management | Difficulty: Hard
// Concepts: fragmentation, external, internal, coalescing
// Description: Distinguish external and internal fragmentation and mitigate each.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Memory Fragmentation (External/Internal)
// Question ID: 403
class MemoryFragmentationExternalInternal {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MemoryFragmentationExternalInternal(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
