// Question #230: Bridge Tables for Many-to-Many
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: bridge table, many-to-many, junction, aggregate
// Description: Model many-to-many relationships with bridge tables and resolve aggregates correctly.
package sql

import "fmt"

// Bridge Tables for Many-to-Many
// Implements a database design pattern for question #230.

// BridgeTablesForManyToMany represents the database schema/concept.
type BridgeTablesForManyToMany struct {
        tables map[string][]string
}

// NewBridgeTablesForManyToMany initializes the schema.
func NewBridgeTablesForManyToMany() *BridgeTablesForManyToMany {
        return &BridgeTablesForManyToMany{tables: make(map[string][]string)}
}
