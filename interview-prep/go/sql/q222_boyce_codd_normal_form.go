// Question #222: Boyce-Codd Normal Form
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: BCNF, functional dependency, candidate key, decomposition
// Description: Identify and decompose a schema to BCNF by removing non-trivial dependencies where a determinant is not a candidate key.
package sql

import "fmt"

// Boyce-Codd Normal Form
// Implements a database design pattern for question #222.

// BoyceCoddNormalForm represents the database schema/concept.
type BoyceCoddNormalForm struct {
        tables map[string][]string
}

// NewBoyceCoddNormalForm initializes the schema.
func NewBoyceCoddNormalForm() *BoyceCoddNormalForm {
        return &BoyceCoddNormalForm{tables: make(map[string][]string)}
}
