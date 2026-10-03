// Question #257: Index Range Scan vs Skip Scan
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: range scan, skip scan, composite index, leading column
// Description: Contrast range scans with skip scans that handle leading-column equality filters.
package sql

import "fmt"

// Index Range Scan vs Skip Scan
// Implements a database design pattern for question #257.

// IndexRangeScanVsSkipScan represents the database schema/concept.
type IndexRangeScanVsSkipScan struct {
        tables map[string][]string
}

// NewIndexRangeScanVsSkipScan initializes the schema.
func NewIndexRangeScanVsSkipScan() *IndexRangeScanVsSkipScan {
        return &IndexRangeScanVsSkipScan{tables: make(map[string][]string)}
}
