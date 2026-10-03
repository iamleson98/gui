// Question #50: Phaser / Cyclic Barrier
// Category: Concurrency | Difficulty: Hard
// Concepts: phaser, cyclic barrier, parties, phases
// Description: Design a phaser supporting dynamic party registration, arrivals, and phase advancement.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Phaser / Cyclic Barrier
// Question ID: 50
class PhaserCyclicBarrier {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
