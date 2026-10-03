// Question #60: Coherence Traffic and False Sharing
// Category: Concurrency | Difficulty: Hard
// Concepts: false sharing, cache line, padding, coherence
// Description: Diagnose false sharing between adjacent atomics on the same cache line and pad to eliminate coherence traffic.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Coherence Traffic and False Sharing
// Question ID: 60
class CoherenceTrafficAndFalseSharing {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
