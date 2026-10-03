// Question #30: Counting Semaphore
// Category: Concurrency | Difficulty: Hard | Concepts: counting, parking, fast/slow path
#pragma once
#include <mutex>
#include <condition_variable>

namespace interview_prep {

class Semaphore {
    std::mutex mu_;
    std::condition_variable cv_;
    size_t count_;

public:
    explicit Semaphore(size_t n) : count_(n) {}

    void acquire() {
        std::unique_lock<std::mutex> lk(mu_);
        cv_.wait(lk, [this]{ return count_ > 0; });
        count_--;
    }

    void release() {
        std::lock_guard<std::mutex> lk(mu_);
        count_++;
        cv_.notify_one();
    }

    bool try_acquire() {
        std::lock_guard<std::mutex> lk(mu_);
        if (count_ > 0) { count_--; return true; }
        return false;
    }
};

} // namespace interview_prep
