// Question #242: Pivoting and Unpivoting
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: pivot, unpivot, conditional aggregation, cross tab
// Description: Pivot rows to columns and unpivot columns to rows using conditional aggregation and UNION.
package sql


// Pivoting and Unpivoting
// Implements a database design pattern for question #242.

// Q242_PivotingAndUnpivoting represents the database schema/concept.
type Q242_PivotingAndUnpivoting struct {
        tables map[string][]string
}

// NewQ242_PivotingAndUnpivoting initializes the schema.
func NewQ242_PivotingAndUnpivoting() *Q242_PivotingAndUnpivoting {
        return &Q242_PivotingAndUnpivoting{tables: make(map[string][]string)}
}
