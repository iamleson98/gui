// Question #274: Gap Locks and Next-Key Locking
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: gap lock, next-key, phantom, InnoDB
// Description: Prevent phantom reads with gap and next-key locking in InnoDB repeatable-read.
package sql


// Gap Locks and Next-Key Locking
// Implements a database design pattern for question #274.

// Q274_GapLocksAndNextKeyLocking represents the database schema/concept.
type Q274_GapLocksAndNextKeyLocking struct {
        tables map[string][]string
}

// NewQ274_GapLocksAndNextKeyLocking initializes the schema.
func NewQ274_GapLocksAndNextKeyLocking() *Q274_GapLocksAndNextKeyLocking {
        return &Q274_GapLocksAndNextKeyLocking{tables: make(map[string][]string)}
}
