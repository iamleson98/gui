// Question #283: Event Sourcing
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: event sourcing, events, projection, replay
// Description: Store domain events as the source of truth and project read models from the event log.
package sql


// Event Sourcing
// Implements a database design pattern for question #283.

// Q283_EventSourcing represents the database schema/concept.
type Q283_EventSourcing struct {
        tables map[string][]string
}

// NewQ283_EventSourcing initializes the schema.
func NewQ283_EventSourcing() *Q283_EventSourcing {
        return &Q283_EventSourcing{tables: make(map[string][]string)}
}
