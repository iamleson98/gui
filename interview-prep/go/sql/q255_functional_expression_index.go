// Question #255: Functional/Expression Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: expression index, function, immutable, predicate
// Description: Index the result of an expression or function to accelerate transformed predicates.
package sql

import "fmt"

// Functional/Expression Index
// Implements a database design pattern for question #255.

// FunctionalExpressionIndex represents the database schema/concept.
type FunctionalExpressionIndex struct {
        tables map[string][]string
}

// NewFunctionalExpressionIndex initializes the schema.
func NewFunctionalExpressionIndex() *FunctionalExpressionIndex {
        return &FunctionalExpressionIndex{tables: make(map[string][]string)}
}
