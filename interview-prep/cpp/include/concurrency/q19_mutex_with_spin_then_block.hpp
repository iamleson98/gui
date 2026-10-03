// Question #19: Mutex with Spin-then-Block
// Category: Concurrency | Difficulty: Hard
// Concepts: mutex, spin-then-block, futex, latency
// Description: Design a mutex that spins briefly in userspace and only then issues a system call to park the thread.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Mutex with Spin-then-Block
// Question ID: 19
class MutexWithSpinThenBlock {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
