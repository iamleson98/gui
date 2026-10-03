// Question #283: Event Sourcing
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: event sourcing, events, projection, replay
// Description: Store domain events as the source of truth and project read models from the event log.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Event Sourcing
// Question ID: 283
class EventSourcing {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
