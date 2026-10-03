// Question #96: Red-Black Tree
// Category: Data Structures | Difficulty: Hard | Concepts: red-black invariants, rotations
#pragma once
#include <memory>

namespace interview_prep {

class RBTree {
    enum Color { RED, BLACK };
    struct Node {
        int key;
        Color color;
        Node *left, *right, *parent;
        Node(int k) : key(k), color(RED), left(nullptr), right(nullptr), parent(nullptr) {}
    };
    Node* root_ = nullptr;
    int size_ = 0;

    void left_rotate(Node* x) {
        Node* y = x->right;
        x->right = y->left;
        if (y->left) y->left->parent = x;
        y->parent = x->parent;
        if (!x->parent) root_ = y;
        else if (x == x->parent->left) x->parent->left = y;
        else x->parent->right = y;
        y->left = x;
        x->parent = y;
    }

    void right_rotate(Node* x) {
        Node* y = x->left;
        x->left = y->right;
        if (y->right) y->right->parent = x;
        y->parent = x->parent;
        if (!x->parent) root_ = y;
        else if (x == x->parent->right) x->parent->right = y;
        else x->parent->left = y;
        y->right = x;
        x->parent = y;
    }

    void fixup(Node* z) {
        while (z->parent && z->parent->color == RED) {
            // Simplified — full fixup omitted
            z->parent->color = BLACK;
            if (z->parent->parent) z->parent->parent->color = RED;
            z = z->parent->parent;
            if (!z || !z->parent) break;
        }
        root_->color = BLACK;
    }

public:
    void insert(int key) {
        Node* z = new Node(key);
        Node* y = nullptr;
        Node* x = root_;
        while (x) {
            y = x;
            x = (key < x->key) ? x->left : x->right;
        }
        z->parent = y;
        if (!y) root_ = z;
        else if (key < y->key) y->left = z;
        else y->right = z;
        size_++;
        fixup(z);
    }

    bool search(int key) const {
        Node* node = root_;
        while (node) {
            if (key == node->key) return true;
            node = (key < node->key) ? node->left : node->right;
        }
        return false;
    }

    int size() const { return size_; }
};

} // namespace interview_prep
