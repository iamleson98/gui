// Question #230: Bridge Tables for Many-to-Many
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: bridge table, many-to-many, junction, aggregate
// Description: Model many-to-many relationships with bridge tables and resolve aggregates correctly.
package sql


// Bridge Tables for Many-to-Many
// Implements a database design pattern for question #230.

// Q230_BridgeTablesForManyToMany represents the database schema/concept.
type Q230_BridgeTablesForManyToMany struct {
        tables map[string][]string
}

// NewQ230_BridgeTablesForManyToMany initializes the schema.
func NewQ230_BridgeTablesForManyToMany() *Q230_BridgeTablesForManyToMany {
        return &Q230_BridgeTablesForManyToMany{tables: make(map[string][]string)}
}
