// Question #257: Index Range Scan vs Skip Scan
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: range scan, skip scan, composite index, leading column
// Description: Contrast range scans with skip scans that handle leading-column equality filters.
package sql


// Index Range Scan vs Skip Scan
// Implements a database design pattern for question #257.

// Q257_IndexRangeScanVsSkipScan represents the database schema/concept.
type Q257_IndexRangeScanVsSkipScan struct {
        tables map[string][]string
}

// NewQ257_IndexRangeScanVsSkipScan initializes the schema.
func NewQ257_IndexRangeScanVsSkipScan() *Q257_IndexRangeScanVsSkipScan {
        return &Q257_IndexRangeScanVsSkipScan{tables: make(map[string][]string)}
}
