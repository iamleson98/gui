// Question #224: Denormalization Tradeoffs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: denormalization, read performance, anomalies, tradeoff
// Description: Evaluate when denormalizing for read performance outweighs the cost of update anomalies.
package sql


// Denormalization Tradeoffs
// Implements a database design pattern for question #224.

// Q224_DenormalizationTradeoffs represents the database schema/concept.
type Q224_DenormalizationTradeoffs struct {
        tables map[string][]string
}

// NewQ224_DenormalizationTradeoffs initializes the schema.
func NewQ224_DenormalizationTradeoffs() *Q224_DenormalizationTradeoffs {
        return &Q224_DenormalizationTradeoffs{tables: make(map[string][]string)}
}
