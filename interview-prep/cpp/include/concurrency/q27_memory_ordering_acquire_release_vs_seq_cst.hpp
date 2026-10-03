// Question #27: Memory Ordering: Acquire/Release vs seq_cst
// Category: Concurrency | Difficulty: Hard
// Concepts: memory ordering, acquire/release, seq_cst, reordered
// Description: Compare acquire/release, relaxed, and sequentially-consistent ordering and pick the weakest safe ordering per access.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Memory Ordering: Acquire/Release vs seq_cst
// Question ID: 27
class MemoryOrderingAcquireReleaseVsSeqCst {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
