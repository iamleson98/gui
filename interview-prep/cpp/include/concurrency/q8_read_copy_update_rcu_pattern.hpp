// Question #8: Read-Copy-Update (RCU) Pattern
// Category: Concurrency | Difficulty: Hard
// Concepts: RCU, grace period, deferred reclamation, read-mostly
// Description: Simulate RCU by allowing readers to proceed without locks and deferring reclamation to a grace period.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Read-Copy-Update (RCU) Pattern
// Question ID: 8
class ReadCopyUpdateRcuPattern {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
