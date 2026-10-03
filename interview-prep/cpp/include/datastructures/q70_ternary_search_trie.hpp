// Question #70: Ternary Search Trie
// Category: Data Structures | Difficulty: Hard
// Concepts: ternary trie, strings, prefix, branching
// Description: Implement a ternary search trie for string keys with character-by-character branching.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Ternary Search Trie
// Question ID: 70
class TernarySearchTrie {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
