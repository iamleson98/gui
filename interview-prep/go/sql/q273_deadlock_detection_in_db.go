// Question #273: Deadlock Detection in DB
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: deadlock, wait-for graph, detection, victim
// Description: Use a wait-for graph to detect and resolve deadlocks among transactions.
package sql

import "fmt"

// Deadlock Detection in DB
// Implements a database design pattern for question #273.

// DeadlockDetectionInDb represents the database schema/concept.
type DeadlockDetectionInDb struct {
        tables map[string][]string
}

// NewDeadlockDetectionInDb initializes the schema.
func NewDeadlockDetectionInDb() *DeadlockDetectionInDb {
        return &DeadlockDetectionInDb{tables: make(map[string][]string)}
}
