// Question #18: Futex (Fast Userspace Mutex)
// Category: Concurrency | Difficulty: Hard
// Concepts: futex, mutex, kernel parking, wait queue
// Description: Implement a userspace mutex that spins on an atomic word and parks in the kernel only on contention.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Futex (Fast Userspace Mutex)
// Question ID: 18
class FutexFastUserspaceMutex {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
