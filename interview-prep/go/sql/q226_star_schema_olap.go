// Question #226: Star Schema (OLAP)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: star schema, fact table, dimensions, OLAP
// Description: Design a star schema with a central fact table surrounded by dimension tables for OLAP queries.
package sql


// Star Schema (OLAP)
// Implements a database design pattern for question #226.

// Q226_StarSchemaOlap represents the database schema/concept.
type Q226_StarSchemaOlap struct {
        tables map[string][]string
}

// NewQ226_StarSchemaOlap initializes the schema.
func NewQ226_StarSchemaOlap() *Q226_StarSchemaOlap {
        return &Q226_StarSchemaOlap{tables: make(map[string][]string)}
}
