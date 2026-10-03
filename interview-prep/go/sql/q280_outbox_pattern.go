// Question #280: Outbox Pattern
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: outbox, exactly-once, event publishing, transactional
// Description: Reliably publish events to a broker by writing them transactionally to an outbox table.
package sql


// Outbox Pattern
// Implements a database design pattern for question #280.

// Q280_OutboxPattern represents the database schema/concept.
type Q280_OutboxPattern struct {
        tables map[string][]string
}

// NewQ280_OutboxPattern initializes the schema.
func NewQ280_OutboxPattern() *Q280_OutboxPattern {
        return &Q280_OutboxPattern{tables: make(map[string][]string)}
}
