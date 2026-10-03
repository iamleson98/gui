// Question #253: Covering Index (INCLUDE)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: covering index, INCLUDE, index-only scan, visibility map
// Description: Add included non-key columns to make an index covering and enable index-only scans.
package sql


// Covering Index (INCLUDE)
// Implements a database design pattern for question #253.

// Q253_CoveringIndexInclude represents the database schema/concept.
type Q253_CoveringIndexInclude struct {
        tables map[string][]string
}

// NewQ253_CoveringIndexInclude initializes the schema.
func NewQ253_CoveringIndexInclude() *Q253_CoveringIndexInclude {
        return &Q253_CoveringIndexInclude{tables: make(map[string][]string)}
}
