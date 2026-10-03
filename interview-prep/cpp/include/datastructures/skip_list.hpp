// Question #84: Skip List
// Category: Data Structures | Difficulty: Hard | Concepts: geometric distribution, levels
#pragma once
#include <vector>
#include <random>
#include <cstdint>

namespace interview_prep {

template <typename K, typename V, typename Compare = std::less<K>>
class SkipList {
    static const int MAX_LEVEL = 32;
    struct Node {
        K key; V value; std::vector<Node*> next;
        Node(int level) : next(level, nullptr) {}
    };
    Node* head_;
    Compare cmp_;
    int len_{0};
    std::mt19937 rng_{42};

    int random_level() {
        int lvl = 1;
        while ((rng_() & 1) && lvl < MAX_LEVEL) lvl++;
        return lvl;
    }

public:
    SkipList() : head_(new Node(MAX_LEVEL)) {}
    ~SkipList() { /* cleanup omitted */ }

    void insert(const K& key, const V& value) {
        std::vector<Node*> update(MAX_LEVEL, nullptr);
        Node* curr = head_;
        for (int i = MAX_LEVEL - 1; i >= 0; --i) {
            while (curr->next[i] && cmp_(curr->next[i]->key, key))
                curr = curr->next[i];
            update[i] = curr;
        }
        curr = curr->next[0];
        if (curr && !cmp_(key, curr->key) && !cmp_(curr->key, key)) {
            curr->value = value;
            return;
        }
        int lvl = random_level();
        Node* node = new Node(lvl);
        node->key = key;
        node->value = value;
        for (int i = 0; i < lvl; ++i) {
            node->next[i] = update[i]->next[i];
            update[i]->next[i] = node;
        }
        len_++;
    }

    bool search(const K& key, V& out) {
        Node* curr = head_;
        for (int i = MAX_LEVEL - 1; i >= 0; --i) {
            while (curr->next[i] && cmp_(curr->next[i]->key, key))
                curr = curr->next[i];
        }
        curr = curr->next[0];
        if (curr && !cmp_(key, curr->key) && !cmp_(curr->key, key)) {
            out = curr->value;
            return true;
        }
        return false;
    }

    int len() const { return len_; }
};

} // namespace interview_prep
