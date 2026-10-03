// Question #354: Design a Merkle Tree for Anti-Entropy
// Category: System Design | Difficulty: Hard
// Concepts: Merkle tree, anti-entropy, replica diff, repair
// Description: Compare replicas with Merkle trees to localize and repair differences.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design a Merkle Tree for Anti-Entropy
// Question ID: 354
class DesignAMerkleTreeForAntiEntropy {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
