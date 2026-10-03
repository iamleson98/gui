#pragma once
#include <vector>
#include <cstdint>
#include <cmath>
#include <functional>
#include <string>
#include <algorithm>

namespace interview_prep {

class BloomFilter {
    std::vector<uint64_t> bits_;
    size_t m_;
    int k_;

    size_t hash(const std::string& data, int seed) const {
        std::string seeded = data + std::string(1, (char)(seed + 'A'));
        size_t h1 = std::hash<std::string>{}(data);
        size_t h2 = std::hash<std::string>{}(seeded);
        if (h2 == 0) h2 = 1;
        return (h1 + (size_t)seed * h2) % m_;
    }

public:
    BloomFilter(size_t expected_n, double fp_rate) {
        double n = (double)(expected_n > 0 ? expected_n : 1);
        double p = fp_rate;
        if (p < 0.0001) p = 0.0001;
        if (p > 0.99) p = 0.99;
        m_ = (size_t)std::ceil(-(n * std::log(p)) / (std::log(2.0) * std::log(2.0)));
        k_ = (int)std::ceil((double)m_ / n * std::log(2.0));
        if (k_ < 1) k_ = 1;
        bits_.resize((m_ + 63) / 64, 0);
    }

    void add(const std::string& data) {
        for (int i = 0; i < k_; ++i) {
            size_t idx = hash(data, i);
            bits_[idx / 64] |= (1ULL << (idx % 64));
        }
    }

    bool contains(const std::string& data) const {
        for (int i = 0; i < k_; ++i) {
            size_t idx = hash(data, i);
            if (!(bits_[idx / 64] & (1ULL << (idx % 64)))) return false;
        }
        return true;
    }
};

} // namespace interview_prep
