// Question #227: Snowflake Schema
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: snowflake, normalized dimensions, OLAP, storage
// Description: Normalize dimensions in a star schema to form a snowflake and weigh query vs storage tradeoffs.
package sql


// Snowflake Schema
// Implements a database design pattern for question #227.

// Q227_SnowflakeSchema represents the database schema/concept.
type Q227_SnowflakeSchema struct {
        tables map[string][]string
}

// NewQ227_SnowflakeSchema initializes the schema.
func NewQ227_SnowflakeSchema() *Q227_SnowflakeSchema {
        return &Q227_SnowflakeSchema{tables: make(map[string][]string)}
}
