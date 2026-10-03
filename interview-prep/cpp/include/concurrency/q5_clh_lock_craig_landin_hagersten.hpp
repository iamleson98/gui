// Question #5: CLH Lock (Craig, Landin, Hagersten)
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, queue lock, FIFO, spin locality
// Description: Build a queue lock whose thread spins on the predecessor's lock word and hands off ownership by toggling its own node.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// CLH Lock (Craig, Landin, Hagersten)
// Question ID: 5
class ClhLockCraigLandinHagersten {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
