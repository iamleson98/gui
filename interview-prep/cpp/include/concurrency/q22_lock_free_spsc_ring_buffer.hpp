// Question #22: Lock-Free SPSC Ring Buffer
// Category: Concurrency | Difficulty: Hard
// Concepts: SPSC, ring buffer, memory ordering, cache lines
// Description: Build a single-producer single-consumer bounded ring buffer using relaxed loads/stores and a power-of-two size.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Lock-Free SPSC Ring Buffer
// Question ID: 22
class LockFreeSpscRingBuffer {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
