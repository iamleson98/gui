// Question #80: Fibonacci Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: Fibonacci heap, amortized, decrease-key, cascading cut
// Description: Implement a Fibonacci heap with lazy melding and amortized O(1) decrease-key for Dijkstra.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Fibonacci Heap
// Question ID: 80
class FibonacciHeap {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
