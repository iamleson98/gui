// Question #262: Undo/Redo Logging
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: undo, redo, steal, no-force
// Description: Contrast undo-only, redo-only, and undo-redo logging with respect to steal and no-force policies.
package sql

import "fmt"

// Undo/Redo Logging
// Implements a database design pattern for question #262.

// UndoRedoLogging represents the database schema/concept.
type UndoRedoLogging struct {
        tables map[string][]string
}

// NewUndoRedoLogging initializes the schema.
func NewUndoRedoLogging() *UndoRedoLogging {
        return &UndoRedoLogging{tables: make(map[string][]string)}
}
