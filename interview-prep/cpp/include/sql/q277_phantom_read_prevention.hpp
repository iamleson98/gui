// Question #277: Phantom Read Prevention
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: phantom, predicate lock, gap lock, stability
// Description: Prevent phantom reads via predicate locking or gap locks to keep a predicate result set stable.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Phantom Read Prevention
// Question ID: 277
class PhantomReadPrevention {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
