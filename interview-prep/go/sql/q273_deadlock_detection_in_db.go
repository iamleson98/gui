// Question #273: Deadlock Detection in DB
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: deadlock, wait-for graph, detection, victim
// Description: Use a wait-for graph to detect and resolve deadlocks among transactions.
package sql


// Deadlock Detection in DB
// Implements a database design pattern for question #273.

// Q273_DeadlockDetectionInDb represents the database schema/concept.
type Q273_DeadlockDetectionInDb struct {
        tables map[string][]string
}

// NewQ273_DeadlockDetectionInDb initializes the schema.
func NewQ273_DeadlockDetectionInDb() *Q273_DeadlockDetectionInDb {
        return &Q273_DeadlockDetectionInDb{tables: make(map[string][]string)}
}
