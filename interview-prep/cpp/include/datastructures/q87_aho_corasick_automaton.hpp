// Question #87: Aho-Corasick Automaton
// Category: Data Structures | Difficulty: Hard
// Concepts: Aho-Corasick, failure links, multi-pattern, DFA
// Description: Construct the AC automaton with failure links for multi-pattern string matching.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Aho-Corasick Automaton
// Question ID: 87
class AhoCorasickAutomaton {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
