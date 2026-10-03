// Question #240: Common Table Expressions
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CTE, readability, materialization, recursion
// Description: Refactor complex queries with CTEs for readability and understand materialization behavior.
package sql


// Common Table Expressions
// Implements a database design pattern for question #240.

// Q240_CommonTableExpressions represents the database schema/concept.
type Q240_CommonTableExpressions struct {
        tables map[string][]string
}

// NewQ240_CommonTableExpressions initializes the schema.
func NewQ240_CommonTableExpressions() *Q240_CommonTableExpressions {
        return &Q240_CommonTableExpressions{tables: make(map[string][]string)}
}
