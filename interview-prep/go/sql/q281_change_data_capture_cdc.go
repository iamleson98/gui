// Question #281: Change Data Capture (CDC)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CDC, WAL, streaming, transaction log
// Description: Stream row changes from a database by reading the WAL or transaction log.
package sql

import "fmt"

// Change Data Capture (CDC)
// Implements a database design pattern for question #281.

// ChangeDataCaptureCdc represents the database schema/concept.
type ChangeDataCaptureCdc struct {
        tables map[string][]string
}

// NewChangeDataCaptureCdc initializes the schema.
func NewChangeDataCaptureCdc() *ChangeDataCaptureCdc {
        return &ChangeDataCaptureCdc{tables: make(map[string][]string)}
}
