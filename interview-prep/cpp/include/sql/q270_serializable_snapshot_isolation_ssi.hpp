// Question #270: Serializable Snapshot Isolation (SSI)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: SSI, serializable, conflict, safe retry
// Description: Detect dangerous read/write patterns to provide serializability over snapshot isolation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Serializable Snapshot Isolation (SSI)
// Question ID: 270
class SerializableSnapshotIsolationSsi {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
