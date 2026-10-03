// Question #4: MCS Lock (Mellor-Crummy & Scott)
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, queue lock, scalability, NUMA
// Description: Implement a scalable list-based queue lock where each thread spins on a locally-cached flag.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// MCS Lock (Mellor-Crummy & Scott)
// Question ID: 4
class McsLockMellorCrummyScott {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
