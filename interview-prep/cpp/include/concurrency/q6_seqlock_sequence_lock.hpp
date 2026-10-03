// Question #6: SeqLock (Sequence Lock)
// Category: Concurrency | Difficulty: Hard
// Concepts: seqlock, readers-writers, memory ordering, fences
// Description: Implement a sequence-lock reader/writer pattern allowing lock-free reads while writes increment a counter twice.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// SeqLock (Sequence Lock)
// Question ID: 6
class SeqlockSequenceLock {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
