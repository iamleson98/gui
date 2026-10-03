// Question #124: LSM-Tree (Log-Structured Merge)
// Category: Data Structures | Difficulty: Hard
// Concepts: LSM tree, memtable, SSTable, compaction
// Description: Implement a log-structured merge tree with memtable, SSTables, and leveled compaction.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// LSM-Tree (Log-Structured Merge)
// Question ID: 124
class LsmTreeLogStructuredMerge {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
