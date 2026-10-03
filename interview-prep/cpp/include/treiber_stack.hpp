#pragma once
#include <atomic>
#include <memory>

namespace interview_prep {

// Lock-Free Treiber Stack — CAS-based LIFO stack.
template <typename T>
class TreiberStack {
    struct Node {
        T value;
        Node* next;
        Node(T v) : value(std::move(v)), next(nullptr) {}
    };
    std::atomic<Node*> head_{nullptr};
public:
    TreiberStack() = default;

    TreiberStack(const TreiberStack&) = delete;
    TreiberStack& operator=(const TreiberStack&) = delete;

    ~TreiberStack() {
        T tmp;
        while (pop(tmp)) {}
    }

    void push(T value) {
        Node* node = new Node(std::move(value));
        Node* old;
        do {
            old = head_.load(std::memory_order_acquire);
            node->next = old;
        } while (!head_.compare_exchange_weak(old, node,
            std::memory_order_release, std::memory_order_relaxed));
    }

    bool pop(T& out) {
        Node* old;
        do {
            old = head_.load(std::memory_order_acquire);
            if (!old) return false;
        } while (!head_.compare_exchange_weak(old, old->next,
            std::memory_order_release, std::memory_order_relaxed));
        out = std::move(old->value);
        delete old;
        return true;
    }

    bool empty() const {
        return head_.load(std::memory_order_acquire) == nullptr;
    }
};

} // namespace interview_prep
