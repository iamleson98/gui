// Question #42: Thread Pool with Work Queue
// Category: Concurrency | Difficulty: Hard
// Concepts: thread pool, work queue, workers, shutdown
// Description: Implement a fixed-size worker pool with a global task queue, blocking dequeue, and shutdown semantics.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Thread Pool with Work Queue
// Question ID: 42
class ThreadPoolWithWorkQueue {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
