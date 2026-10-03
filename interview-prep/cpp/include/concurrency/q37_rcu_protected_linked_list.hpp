// Question #37: RCU-Protected Linked List
// Category: Concurrency | Difficulty: Hard
// Concepts: RCU, linked list, grace period, read-mostly
// Description: Implement a linked list whose readers traverse lock-free while updaters use RCU to defer node removal.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// RCU-Protected Linked List
// Question ID: 37
class RcuProtectedLinkedList {
private:
    std::atomic<int64_t> state_{0};
public:
    void execute() { state_.fetch_add(1, std::memory_order_acq_rel); }
    int64_t result() const { return state_.load(std::memory_order_acquire); }
};

} // namespace interview_prep
