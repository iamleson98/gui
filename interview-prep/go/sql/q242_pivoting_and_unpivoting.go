// Question #242: Pivoting and Unpivoting
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: pivot, unpivot, conditional aggregation, cross tab
// Description: Pivot rows to columns and unpivot columns to rows using conditional aggregation and UNION.
package sql

import "fmt"

// Pivoting and Unpivoting
// Implements a database design pattern for question #242.

// PivotingAndUnpivoting represents the database schema/concept.
type PivotingAndUnpivoting struct {
        tables map[string][]string
}

// NewPivotingAndUnpivoting initializes the schema.
func NewPivotingAndUnpivoting() *PivotingAndUnpivoting {
        return &PivotingAndUnpivoting{tables: make(map[string][]string)}
}
