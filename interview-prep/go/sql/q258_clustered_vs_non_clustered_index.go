// Question #258: Clustered vs Non-Clustered Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: clustered, non-clustered, index-organized, secondary
// Description: Distinguish clustered (index-organized) tables from non-clustered secondary indexes.
package sql


// Clustered vs Non-Clustered Index
// Implements a database design pattern for question #258.

// Q258_ClusteredVsNonClusteredIndex represents the database schema/concept.
type Q258_ClusteredVsNonClusteredIndex struct {
        tables map[string][]string
}

// NewQ258_ClusteredVsNonClusteredIndex initializes the schema.
func NewQ258_ClusteredVsNonClusteredIndex() *Q258_ClusteredVsNonClusteredIndex {
        return &Q258_ClusteredVsNonClusteredIndex{tables: make(map[string][]string)}
}
