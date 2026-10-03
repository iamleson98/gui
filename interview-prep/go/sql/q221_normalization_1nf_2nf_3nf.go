// Question #221: Normalization: 1NF/2NF/3NF
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: normalization, 1NF, 2NF, 3NF, anomalies
// Description: Apply first, second, and third normal forms to eliminate anomalies and redundancy in a schema.
package sql

import "fmt"

// Normalization: 1NF/2NF/3NF
// Implements a database design pattern for question #221.

// Normalization1Nf2Nf3Nf represents the database schema/concept.
type Normalization1Nf2Nf3Nf struct {
        tables map[string][]string
}

// NewNormalization1Nf2Nf3Nf initializes the schema.
func NewNormalization1Nf2Nf3Nf() *Normalization1Nf2Nf3Nf {
        return &Normalization1Nf2Nf3Nf{tables: make(map[string][]string)}
}
