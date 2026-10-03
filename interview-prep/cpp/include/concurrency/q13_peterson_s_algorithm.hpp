// Question #13: Peterson's Algorithm
// Category: Concurrency | Difficulty: Hard
// Concepts: mutual exclusion, flags, turn, memory ordering
// Description: Implement the classic two-process mutual exclusion algorithm using flags and a turn variable with sequential consistency.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Peterson's Algorithm
// Question ID: 13
class PetersonSAlgorithm {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
