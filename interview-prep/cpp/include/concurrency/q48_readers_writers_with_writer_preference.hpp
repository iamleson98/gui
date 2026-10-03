// Question #48: Readers-Writers with Writer Preference
// Category: Concurrency | Difficulty: Hard
// Concepts: readers-writers, writer preference, starvation, fairness
// Description: Design an RW lock that prefers writers to avoid writer starvation while preventing reader starvation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Readers-Writers with Writer Preference
// Question ID: 48
class ReadersWritersWithWriterPreference {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
