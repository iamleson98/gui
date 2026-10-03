// Question #280: Outbox Pattern
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: outbox, exactly-once, event publishing, transactional
// Description: Reliably publish events to a broker by writing them transactionally to an outbox table.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Outbox Pattern
// Question ID: 280
class OutboxPattern {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
