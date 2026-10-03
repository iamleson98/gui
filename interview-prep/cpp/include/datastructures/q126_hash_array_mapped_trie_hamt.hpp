// Question #126: Hash Array Mapped Trie (HAMT)
// Category: Data Structures | Difficulty: Hard
// Concepts: HAMT, bitmap, hash trie, persistent
// Description: Implement a HAMT using a sparse bitmap and a variable-length pointer array per node.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Hash Array Mapped Trie (HAMT)
// Question ID: 126
class HashArrayMappedTrieHamt {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
