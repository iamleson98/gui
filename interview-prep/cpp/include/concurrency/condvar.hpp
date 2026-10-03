// Question #16: Condition Variable
// Category: Concurrency | Difficulty: Hard | Concepts: wait/notify, spurious wakeups
#pragma once
#include <mutex>
#include <condition_variable>
#include <functional>

namespace interview_prep {

class CondVar {
    std::mutex mu_;
    std::condition_variable cv_;

public:
    void wait(std::function<bool()> pred) {
        std::unique_lock<std::mutex> lk(mu_);
        cv_.wait(lk, pred);
    }

    void signal() {
        std::lock_guard<std::mutex> lk(mu_);
        cv_.notify_one();
    }

    void broadcast() {
        std::lock_guard<std::mutex> lk(mu_);
        cv_.notify_all();
    }
};

} // namespace interview_prep
