// Question #7: Adaptive Spinlock
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, futex, backoff, hybrid
// Description: Design a spinlock that spins briefly then falls back to a kernel futex or parking primitive to avoid wasted CPU.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Adaptive Spinlock
// Question ID: 7
class AdaptiveSpinlock {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
