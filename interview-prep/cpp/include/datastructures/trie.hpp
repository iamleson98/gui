// Question #85: Compressed Trie (Patricia Trie)
// Category: Data Structures | Difficulty: Hard | Concepts: path compression, sparse keys
#pragma once
#include <unordered_map>
#include <string>

namespace interview_prep {

class Trie {
    struct Node {
        std::unordered_map<char, Node*> children;
        bool is_end = false;
    };
    Node* root_;

public:
    Trie() : root_(new Node()) {}
    ~Trie() { destroy(root_); }

    void insert(const std::string& word) {
        Node* node = root_;
        for (char c : word) {
            if (!node->children.count(c))
                node->children[c] = new Node();
            node = node->children[c];
        }
        node->is_end = true;
    }

    bool search(const std::string& word) const {
        Node* node = root_;
        for (char c : word) {
            auto it = node->children.find(c);
            if (it == node->children.end()) return false;
            node = it->second;
        }
        return node->is_end;
    }

    bool starts_with(const std::string& prefix) const {
        Node* node = root_;
        for (char c : prefix) {
            auto it = node->children.find(c);
            if (it == node->children.end()) return false;
            node = it->second;
        }
        return true;
    }

private:
    void destroy(Node* node) {
        if (!node) return;
        for (auto& [c, child] : node->children)
            destroy(child);
        delete node;
    }
};

} // namespace interview_prep
