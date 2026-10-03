// Question #31: Counting Semaphore with Futex
// Category: Concurrency | Difficulty: Hard
// Concepts: semaphore, futex, fast path, wait queue
// Description: Build a fast counting semaphore whose fast path is an atomic compare and whose slow path parks waiters.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Counting Semaphore with Futex
// Question ID: 31
class CountingSemaphoreWithFutex {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
