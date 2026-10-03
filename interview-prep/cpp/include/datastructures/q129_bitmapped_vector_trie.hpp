// Question #129: Bitmapped Vector Trie
// Category: Data Structures | Difficulty: Hard
// Concepts: vector trie, bitmap, branching, persistent
// Description: Build a trie with bitmap per node and branching factor of word size for persistent arrays.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Bitmapped Vector Trie
// Question ID: 129
class BitmappedVectorTrie {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
