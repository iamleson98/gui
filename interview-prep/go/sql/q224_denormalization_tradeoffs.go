// Question #224: Denormalization Tradeoffs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: denormalization, read performance, anomalies, tradeoff
// Description: Evaluate when denormalizing for read performance outweighs the cost of update anomalies.
package sql

import "fmt"

// Denormalization Tradeoffs
// Implements a database design pattern for question #224.

// DenormalizationTradeoffs represents the database schema/concept.
type DenormalizationTradeoffs struct {
        tables map[string][]string
}

// NewDenormalizationTradeoffs initializes the schema.
func NewDenormalizationTradeoffs() *DenormalizationTradeoffs {
        return &DenormalizationTradeoffs{tables: make(map[string][]string)}
}
