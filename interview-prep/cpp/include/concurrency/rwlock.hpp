// Question #14: Readers-Writer Lock (reader preference)
// Category: Concurrency | Difficulty: Hard | Concepts: RW lock, reader preference, starvation
#pragma once
#include <mutex>
#include <condition_variable>

namespace interview_prep {

class RWLock {
    std::mutex mu_;
    int readers_{0};
    bool writer_{false};
    std::condition_variable cv_;

public:
    void rlock() {
        std::unique_lock<std::mutex> lk(mu_);
        cv_.wait(lk, [this]{ return !writer_; });
        readers_++;
    }

    void runlock() {
        std::unique_lock<std::mutex> lk(mu_);
        readers_--;
        if (readers_ == 0) cv_.notify_all();
    }

    void wlock() {
        std::unique_lock<std::mutex> lk(mu_);
        cv_.wait(lk, [this]{ return !writer_ && readers_ == 0; });
        writer_ = true;
    }

    void wunlock() {
        std::unique_lock<std::mutex> lk(mu_);
        writer_ = false;
        cv_.notify_all();
    }
};

} // namespace interview_prep
