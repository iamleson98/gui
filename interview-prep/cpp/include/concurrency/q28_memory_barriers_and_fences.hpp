// Question #28: Memory Barriers and Fences
// Category: Concurrency | Difficulty: Hard
// Concepts: memory fence, load-store, visibility, portability
// Description: Place read and write fences correctly so that lock-free algorithms publish visibility and consumption in the intended order.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Memory Barriers and Fences
// Question ID: 28
class MemoryBarriersAndFences {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
