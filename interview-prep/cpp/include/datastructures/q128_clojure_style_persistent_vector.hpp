// Question #128: Clojure-style Persistent Vector
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent vector, bitmapped trie, tail, immutability
// Description: Implement a persistent vector using a bitmapped trie indexed by chunks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Clojure-style Persistent Vector
// Question ID: 128
class ClojureStylePersistentVector {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
