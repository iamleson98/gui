// Question #11: Ticket Spinlock
// Category: Concurrency | Difficulty: Hard | Concepts: FIFO fairness, cache-line bounce
#pragma once
#include <atomic>

namespace interview_prep {

class TicketSpinlock {
    std::atomic<uint64_t> next_{0};
    std::atomic<uint64_t> now_{0};

public:
    void lock() {
        uint64_t ticket = next_.fetch_add(1, std::memory_order_relaxed);
        while (now_.load(std::memory_order_acquire) != ticket) {
            // spin
        }
    }

    void unlock() {
        now_.fetch_add(1, std::memory_order_release);
    }

    bool try_lock() {
        uint64_t now = now_.load(std::memory_order_acquire);
        uint64_t next = next_.load(std::memory_order_acquire);
        if (now != next) return false;
        return next_.compare_exchange_weak(next, next + 1, std::memory_order_acq_rel, std::memory_order_relaxed);
    }
};

} // namespace interview_prep
