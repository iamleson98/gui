#pragma once
#include <unordered_map>
#include <list>
#include <cstddef>

namespace interview_prep {

// LRU Cache — O(1) get/put using hash map + doubly-linked list.
template <typename K, typename V>
class LRUCache {
    struct Entry {
        K key;
        V value;
    };
    using ListIt = typename std::list<Entry>::iterator;
    std::list<Entry> order_;
    std::unordered_map<K, ListIt> cache_;
    size_t capacity_;
public:
    explicit LRUCache(size_t capacity) : capacity_(capacity > 0 ? capacity : 1) {}

    bool get(const K& key, V& out) {
        auto it = cache_.find(key);
        if (it == cache_.end()) return false;
        order_.splice(order_.begin(), order_, it->second);
        out = it->second->value;
        return true;
    }

    void put(const K& key, V value) {
        auto it = cache_.find(key);
        if (it != cache_.end()) {
            it->second->value = std::move(value);
            order_.splice(order_.begin(), order_, it->second);
            return;
        }
        if (cache_.size() >= capacity_) {
            auto last = order_.back();
            cache_.erase(last.key);
            order_.pop_back();
        }
        order_.push_front({key, std::move(value)});
        cache_[key] = order_.begin();
    }

    size_t size() const { return cache_.size(); }
};

} // namespace interview_prep
