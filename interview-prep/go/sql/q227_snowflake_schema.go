// Question #227: Snowflake Schema
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: snowflake, normalized dimensions, OLAP, storage
// Description: Normalize dimensions in a star schema to form a snowflake and weigh query vs storage tradeoffs.
package sql

import "fmt"

// Snowflake Schema
// Implements a database design pattern for question #227.

// SnowflakeSchema represents the database schema/concept.
type SnowflakeSchema struct {
        tables map[string][]string
}

// NewSnowflakeSchema initializes the schema.
func NewSnowflakeSchema() *SnowflakeSchema {
        return &SnowflakeSchema{tables: make(map[string][]string)}
}
