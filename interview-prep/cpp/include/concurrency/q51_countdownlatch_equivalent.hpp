// Question #51: CountDownLatch Equivalent
// Category: Concurrency | Difficulty: Hard
// Concepts: latch, one-shot, counter, park
// Description: Implement a one-shot latch that blocks threads until a counter reaches zero.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// CountDownLatch Equivalent
// Question ID: 51
class CountdownlatchEquivalent {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
