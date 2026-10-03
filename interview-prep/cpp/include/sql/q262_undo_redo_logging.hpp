// Question #262: Undo/Redo Logging
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: undo, redo, steal, no-force
// Description: Contrast undo-only, redo-only, and undo-redo logging with respect to steal and no-force policies.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Undo/Redo Logging
// Question ID: 262
class UndoRedoLogging {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
