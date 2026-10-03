// Question #255: Functional/Expression Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: expression index, function, immutable, predicate
// Description: Index the result of an expression or function to accelerate transformed predicates.
package sql


// Functional/Expression Index
// Implements a database design pattern for question #255.

// Q255_FunctionalExpressionIndex represents the database schema/concept.
type Q255_FunctionalExpressionIndex struct {
        tables map[string][]string
}

// NewQ255_FunctionalExpressionIndex initializes the schema.
func NewQ255_FunctionalExpressionIndex() *Q255_FunctionalExpressionIndex {
        return &Q255_FunctionalExpressionIndex{tables: make(map[string][]string)}
}
