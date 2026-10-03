// Question #102: LFU Cache
// Category: Data Structures | Difficulty: Hard | Concepts: frequency buckets, ties
#pragma once
#include <unordered_map>
#include <list>

namespace interview_prep {

class LFUCache {
    struct Entry {
        int key, value;
        int freq;
    };
    using ListIt = std::list<Entry>::iterator;
    int capacity_;
    int min_freq_ = 0;
    std::unordered_map<int, ListIt> cache_;
    std::unordered_map<int, std::list<Entry>> freqs_;

public:
    explicit LFUCache(int capacity) : capacity_(capacity > 0 ? capacity : 1) {}

    int get(int key) {
        auto it = cache_.find(key);
        if (it == cache_.end()) return -1;
        increment(it->second);
        return it->second->value;
    }

    void put(int key, int value) {
        if (capacity_ == 0) return;
        auto it = cache_.find(key);
        if (it != cache_.end()) {
            it->second->value = value;
            increment(it->second);
            return;
        }
        if ((int)cache_.size() >= capacity_) evict();
        Entry e{key, value, 1};
        min_freq_ = 1;
        freqs_[1].push_front(e);
        cache_[key] = freqs_[1].begin();
    }

private:
    void increment(ListIt it) {
        Entry e = *it;  // Copy before erasing
        freqs_[e.freq].erase(it);
        if (e.freq == min_freq_ && freqs_[e.freq].empty()) min_freq_++;
        e.freq = e.freq + 1;
        freqs_[e.freq + 1].push_front(e);
        cache_[e.key] = freqs_[e.freq + 1].begin();
    }

    void evict() {
        auto& list = freqs_[min_freq_];
        if (list.empty()) return;
        cache_.erase(list.back().key);
        list.pop_back();
    }
};

} // namespace interview_prep
