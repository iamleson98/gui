// Question #394: mmap and Virtual Address Space
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap, virtual address, anonymous, file-backed
// Description: Use mmap to map files and anonymous memory into the process address space.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// mmap and Virtual Address Space
// Question ID: 394
class MmapAndVirtualAddressSpace {
private:
    std::vector<void*> pool_;
    size_t capacity_;
public:
    explicit MmapAndVirtualAddressSpace(size_t cap) : capacity_(cap) {}
    void* allocate() { if (pool_.empty()) return nullptr; void* p = pool_.back(); pool_.pop_back(); return p; }
    void deallocate(void* p) { if (pool_.size() < capacity_) pool_.push_back(p); }
};

} // namespace interview_prep
