// Question #25: ABA Problem and Tagged Pointers
// Category: Concurrency | Difficulty: Hard
// Concepts: ABA, tagged pointer, CAS, versioning
// Description: Demonstrate the ABA problem on a Treiber stack and fix it using a tagged pointer packing a version counter.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ABA Problem and Tagged Pointers
// Question ID: 25
class AbaProblemAndTaggedPointers {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
