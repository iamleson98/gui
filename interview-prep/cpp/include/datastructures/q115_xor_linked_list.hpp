// Question #115: XOR Linked List
// Category: Data Structures | Difficulty: Hard
// Concepts: XOR list, pointer compression, memory, traversal
// Description: Implement a doubly linked list using XOR of adjacent pointers to store one pointer per node.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// XOR Linked List
// Question ID: 115
class XorLinkedList {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
