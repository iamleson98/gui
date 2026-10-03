// Question #283: Event Sourcing
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: event sourcing, events, projection, replay
// Description: Store domain events as the source of truth and project read models from the event log.
package sql

import "fmt"

// Event Sourcing
// Implements a database design pattern for question #283.

// EventSourcing represents the database schema/concept.
type EventSourcing struct {
        tables map[string][]string
}

// NewEventSourcing initializes the schema.
func NewEventSourcing() *EventSourcing {
        return &EventSourcing{tables: make(map[string][]string)}
}
