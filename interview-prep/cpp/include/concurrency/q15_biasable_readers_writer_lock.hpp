// Question #15: Biasable Readers-Writer Lock
// Category: Concurrency | Difficulty: Hard
// Concepts: readers-writers, biasing, throughput, fairness
// Description: Design an RW lock that can be biased toward readers or writers and rebiased at runtime to tune throughput.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Biasable Readers-Writer Lock
// Question ID: 15
class BiasableReadersWriterLock {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
