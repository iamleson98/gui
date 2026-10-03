// Question #111: Chunking Deque
// Category: Data Structures | Difficulty: Hard
// Concepts: deque, chunking, amortized, persistent
// Description: Build a deque over fixed-size chunks (a 'banker's deque') for amortized O(1) operations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Chunking Deque
// Question ID: 111
class ChunkingDeque {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
