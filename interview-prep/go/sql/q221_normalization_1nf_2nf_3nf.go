// Question #221: Normalization: 1NF/2NF/3NF
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: normalization, 1NF, 2NF, 3NF, anomalies
// Description: Apply first, second, and third normal forms to eliminate anomalies and redundancy in a schema.
package sql


// Normalization: 1NF/2NF/3NF
// Implements a database design pattern for question #221.

// Q221_Normalization1Nf2Nf3Nf represents the database schema/concept.
type Q221_Normalization1Nf2Nf3Nf struct {
        tables map[string][]string
}

// NewQ221_Normalization1Nf2Nf3Nf initializes the schema.
func NewQ221_Normalization1Nf2Nf3Nf() *Q221_Normalization1Nf2Nf3Nf {
        return &Q221_Normalization1Nf2Nf3Nf{tables: make(map[string][]string)}
}
