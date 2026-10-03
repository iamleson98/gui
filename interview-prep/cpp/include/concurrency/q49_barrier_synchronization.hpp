// Question #49: Barrier Synchronization
// Category: Concurrency | Difficulty: Hard
// Concepts: barrier, sense reversal, reuse, wait
// Description: Implement a reusable barrier where N threads wait and then all proceed, with sense reversal.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Barrier Synchronization
// Question ID: 49
class BarrierSynchronization {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
