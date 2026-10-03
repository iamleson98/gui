// Question #231: Surrogate vs Natural Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: surrogate key, natural key, stability, joins
// Description: Choose between surrogate and natural keys, weighing stability, size, and join performance.
package sql

import "fmt"

// Surrogate vs Natural Keys
// Implements a database design pattern for question #231.

// SurrogateVsNaturalKeys represents the database schema/concept.
type SurrogateVsNaturalKeys struct {
        tables map[string][]string
}

// NewSurrogateVsNaturalKeys initializes the schema.
func NewSurrogateVsNaturalKeys() *SurrogateVsNaturalKeys {
        return &SurrogateVsNaturalKeys{tables: make(map[string][]string)}
}
