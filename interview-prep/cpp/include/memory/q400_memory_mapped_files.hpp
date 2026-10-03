// Question #400: Memory-Mapped Files
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap files, zero-copy, page cache, I/O
// Description: Access files through mapped pages for zero-copy I/O and kernel-managed caching.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Memory-Mapped Files
// Question ID: 400
class MemoryMappedFiles {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MemoryMappedFiles(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
