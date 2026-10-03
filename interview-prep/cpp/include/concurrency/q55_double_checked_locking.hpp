// Question #55: Double-Checked Locking
// Category: Concurrency | Difficulty: Hard
// Concepts: double-checked locking, singleton, memory ordering, fence
// Description: Implement a correct double-checked locking singleton using acquire/release fences to avoid the classic pitfall.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Double-Checked Locking
// Question ID: 55
class DoubleCheckedLocking {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
