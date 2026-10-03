// Question #235: Triggers (Before/After)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: triggers, before/after, audit, side effects
// Description: Implement before- and after-triggers for auditing and derived-column maintenance, noting pitfalls.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Triggers (Before/After)
// Question ID: 235
class TriggersBeforeAfter {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
