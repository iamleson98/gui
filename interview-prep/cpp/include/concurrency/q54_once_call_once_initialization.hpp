// Question #54: Once / Call-Once Initialization
// Category: Concurrency | Difficulty: Hard
// Concepts: once, initialization, double-checked, atomic
// Description: Implement std::once / sync.Once semantics guaranteeing a function runs exactly once under concurrency.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Once / Call-Once Initialization
// Question ID: 54
class OnceCallOnceInitialization {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
