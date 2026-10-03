// Question #82: B-Tree
// Category: Data Structures | Difficulty: Hard | Concepts: splitting, bulk loading, range scan
#pragma once
#include <vector>
#include <algorithm>

namespace interview_prep {

class BTree {
    static const int T = 4;
    struct Node {
        std::vector<int> keys;
        std::vector<Node*> children;
        bool leaf;
        Node(bool l) : leaf(l) {}
    };
    Node* root_ = nullptr;

    void split_child(Node* parent, int i) {
        Node* child = parent->children[i];
        Node* new_node = new Node(child->leaf);
        int mid = T - 1;
        for (int j = 0; j < T - 1; ++j)
            new_node->keys.push_back(child->keys[T + j]);
        if (!child->leaf)
            for (int j = 0; j < T; ++j)
                new_node->children.push_back(child->children[T + j]);
        child->keys.resize(mid);
        if (!child->leaf) child->children.resize(T);
        parent->keys.insert(parent->keys.begin() + i, child->keys[mid]);
        parent->children.insert(parent->children.begin() + i + 1, new_node);
    }

    void insert_non_full(Node* node, int key) {
        int i = node->keys.size() - 1;
        if (node->leaf) {
            node->keys.push_back(0);
            while (i >= 0 && key < node->keys[i]) {
                node->keys[i+1] = node->keys[i];
                i--;
            }
            node->keys[i+1] = key;
        } else {
            while (i >= 0 && key < node->keys[i]) i--;
            i++;
            if ((int)node->children[i]->keys.size() >= 2*T-1) {
                split_child(node, i);
                if (key > node->keys[i]) i++;
            }
            insert_non_full(node->children[i], key);
        }
    }

    bool search(Node* node, int key) const {
        if (!node) return false;
        int i = std::lower_bound(node->keys.begin(), node->keys.end(), key) - node->keys.begin();
        if (i < (int)node->keys.size() && node->keys[i] == key) return true;
        if (node->leaf) return false;
        return search(node->children[i], key);
    }

public:
    void insert(int key) {
        if (!root_) root_ = new Node(true);
        if ((int)root_->keys.size() >= 2*T-1) {
            Node* new_root = new Node(false);
            new_root->children.push_back(root_);
            split_child(new_root, 0);
            root_ = new_root;
        }
        insert_non_full(root_, key);
    }

    bool search(int key) const { return search(root_, key); }
};

} // namespace interview_prep
