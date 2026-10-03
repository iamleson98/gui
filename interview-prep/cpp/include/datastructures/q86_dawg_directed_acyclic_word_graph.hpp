// Question #86: DAWG (Directed Acyclic Word Graph)
// Category: Data Structures | Difficulty: Hard
// Concepts: DAWG, minimal DFA, suffix links, strings
// Description: Build a minimal acyclic DFA accepting all suffixes of a string via suffix-tree compression.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// DAWG (Directed Acyclic Word Graph)
// Question ID: 86
class DawgDirectedAcyclicWordGraph {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
