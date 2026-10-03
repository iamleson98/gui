// Question #1: Lock-Free MPSC Queue
// Category: Concurrency | Difficulty: Hard | Concepts: CAS, MPSC, ABA
// Description: Single-producer, multi-consumer queue using atomic CAS.
#pragma once
#include <atomic>
#include <memory>

namespace interview_prep {

template <typename T>
class MpscQueue {
    struct Node {
        T value;
        std::atomic<Node*> next{nullptr};
    };
    std::atomic<Node*> head_{nullptr};
    std::atomic<Node*> tail_{nullptr};
    Node stub_;

public:
    MpscQueue() {
        head_.store(&stub_);
        tail_.store(&stub_);
    }

    ~MpscQueue() {
        T tmp;
        while (dequeue(tmp)) {}
    }

    void enqueue(T value) {
        Node* node = new Node{std::move(value), {}};
        Node* old = head_.exchange(node, std::memory_order_acq_rel);
        old->next.store(node, std::memory_order_release);
    }

    bool dequeue(T& out) {
        Node* tail = tail_.load(std::memory_order_acquire);
        Node* next = tail->next.load(std::memory_order_acquire);
        if (!next) return false;
        out = std::move(next->value);
        tail_.store(next, std::memory_order_release);
        if (tail != &stub_) delete tail;
        return true;
    }
};

} // namespace interview_prep
