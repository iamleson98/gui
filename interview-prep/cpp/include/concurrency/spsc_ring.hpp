// Question #32: SPSC Ring Buffer
// Category: Concurrency | Difficulty: Hard | Concepts: atomic head/tail, power-of-two, cache-line padding
#pragma once
#include <atomic>
#include <vector>

namespace interview_prep {

template <typename T>
class SpscRing {
    std::vector<T> buf_;
    size_t mask_;
    char pad1_[64];
    std::atomic<size_t> head_{0};
    char pad2_[64];
    std::atomic<size_t> tail_{0};

public:
    SpscRing(size_t capacity) {
        size_t cap = 1;
        while (cap < capacity) cap <<= 1;
        buf_.resize(cap);
        mask_ = cap - 1;
    }

    bool try_enqueue(const T& v) {
        size_t h = head_.load(std::memory_order_relaxed);
        size_t t = tail_.load(std::memory_order_acquire);
        if (h - t > mask_) return false;
        buf_[h & mask_] = v;
        head_.store(h + 1, std::memory_order_release);
        return true;
    }

    bool try_dequeue(T& out) {
        size_t t = tail_.load(std::memory_order_relaxed);
        size_t h = head_.load(std::memory_order_acquire);
        if (h == t) return false;
        out = buf_[t & mask_];
        tail_.store(t + 1, std::memory_order_release);
        return true;
    }

    size_t size() const {
        return head_.load(std::memory_order_relaxed) - tail_.load(std::memory_order_relaxed);
    }
};

} // namespace interview_prep
