// Question #225: Entity-Relationship Modeling
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: ER modeling, entities, relationships, cardinality
// Description: Translate an ER diagram into a normalized relational schema with keys and cardinalities.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Entity-Relationship Modeling
// Question ID: 225
class EntityRelationshipModeling {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
