// Question #2: Michael-Scott Lock-Free MPMC Queue
// Category: Concurrency | Difficulty: Hard | Concepts: CAS, dummy node, ABA
#pragma once
#include <atomic>

namespace interview_prep {

template <typename T>
class MSQueue {
    struct Node {
        T value;
        std::atomic<Node*> next{nullptr};
    };
    std::atomic<Node*> head_{nullptr};
    std::atomic<Node*> tail_{nullptr};
    Node* dummy_;

public:
    MSQueue() {
        dummy_ = new Node{};
        head_.store(dummy_);
        tail_.store(dummy_);
    }

    void enqueue(T value) {
        Node* node = new Node{std::move(value), nullptr};
        Node* tail;
        while (true) {
            tail = tail_.load(std::memory_order_acquire);
            Node* next = tail->next.load(std::memory_order_acquire);
            if (tail == tail_.load(std::memory_order_acquire)) {
                if (!next) {
                    Node* null = nullptr;
                    if (tail->next.compare_exchange_weak(null, node, std::memory_order_release, std::memory_order_relaxed)) {
                        tail_.compare_exchange_weak(tail, node, std::memory_order_release, std::memory_order_relaxed);
                        return;
                    }
                } else {
                    tail_.compare_exchange_weak(tail, next, std::memory_order_release, std::memory_order_relaxed);
                }
            }
        }
    }

    bool dequeue(T& out) {
        Node* head;
        while (true) {
            head = head_.load(std::memory_order_acquire);
            Node* tail = tail_.load(std::memory_order_acquire);
            Node* next = head->next.load(std::memory_order_acquire);
            if (head == head_.load(std::memory_order_acquire)) {
                if (head == tail) {
                    if (!next) return false;
                    tail_.compare_exchange_weak(tail, next, std::memory_order_release, std::memory_order_relaxed);
                } else {
                    out = next->value;
                    if (head_.compare_exchange_weak(head, next, std::memory_order_release, std::memory_order_relaxed)) {
                        if (head != dummy_) delete head;
                        return true;
                    }
                }
            }
        }
    }
};

} // namespace interview_prep
