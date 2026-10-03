// Question #222: Boyce-Codd Normal Form
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: BCNF, functional dependency, candidate key, decomposition
// Description: Identify and decompose a schema to BCNF by removing non-trivial dependencies where a determinant is not a candidate key.
package sql


// Boyce-Codd Normal Form
// Implements a database design pattern for question #222.

// Q222_BoyceCoddNormalForm represents the database schema/concept.
type Q222_BoyceCoddNormalForm struct {
        tables map[string][]string
}

// NewQ222_BoyceCoddNormalForm initializes the schema.
func NewQ222_BoyceCoddNormalForm() *Q222_BoyceCoddNormalForm {
        return &Q222_BoyceCoddNormalForm{tables: make(map[string][]string)}
}
