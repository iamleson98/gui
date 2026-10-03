// Question #258: Clustered vs Non-Clustered Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: clustered, non-clustered, index-organized, secondary
// Description: Distinguish clustered (index-organized) tables from non-clustered secondary indexes.
package sql

import "fmt"

// Clustered vs Non-Clustered Index
// Implements a database design pattern for question #258.

// ClusteredVsNonClusteredIndex represents the database schema/concept.
type ClusteredVsNonClusteredIndex struct {
        tables map[string][]string
}

// NewClusteredVsNonClusteredIndex initializes the schema.
func NewClusteredVsNonClusteredIndex() *ClusteredVsNonClusteredIndex {
        return &ClusteredVsNonClusteredIndex{tables: make(map[string][]string)}
}
