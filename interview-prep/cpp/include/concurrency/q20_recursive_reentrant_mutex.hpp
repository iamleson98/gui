// Question #20: Recursive (Reentrant) Mutex
// Category: Concurrency | Difficulty: Hard
// Concepts: mutex, reentrant, owner, recursion count
// Description: Implement a mutex that allows the same thread to acquire it multiple times by tracking an owner and recursion count.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Recursive (Reentrant) Mutex
// Question ID: 20
class RecursiveReentrantMutex {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
