// Question #226: Star Schema (OLAP)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: star schema, fact table, dimensions, OLAP
// Description: Design a star schema with a central fact table surrounded by dimension tables for OLAP queries.
package sql

import "fmt"

// Star Schema (OLAP)
// Implements a database design pattern for question #226.

// StarSchemaOlap represents the database schema/concept.
type StarSchemaOlap struct {
        tables map[string][]string
}

// NewStarSchemaOlap initializes the schema.
func NewStarSchemaOlap() *StarSchemaOlap {
        return &StarSchemaOlap{tables: make(map[string][]string)}
}
