// Question #106: Link-Cut Dynamic Connectivity
// Category: Data Structures | Difficulty: Hard
// Concepts: dynamic connectivity, link-cut, fully dynamic, forest
// Description: Use link-cut trees to maintain connected components under edge insertions and deletions.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Link-Cut Dynamic Connectivity
// Question ID: 106
class LinkCutDynamicConnectivity {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
