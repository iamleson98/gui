// Question #253: Covering Index (INCLUDE)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: covering index, INCLUDE, index-only scan, visibility map
// Description: Add included non-key columns to make an index covering and enable index-only scans.
package sql

import "fmt"

// Covering Index (INCLUDE)
// Implements a database design pattern for question #253.

// CoveringIndexInclude represents the database schema/concept.
type CoveringIndexInclude struct {
        tables map[string][]string
}

// NewCoveringIndexInclude initializes the schema.
func NewCoveringIndexInclude() *CoveringIndexInclude {
        return &CoveringIndexInclude{tables: make(map[string][]string)}
}
