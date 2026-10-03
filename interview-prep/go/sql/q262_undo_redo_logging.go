// Question #262: Undo/Redo Logging
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: undo, redo, steal, no-force
// Description: Contrast undo-only, redo-only, and undo-redo logging with respect to steal and no-force policies.
package sql


// Undo/Redo Logging
// Implements a database design pattern for question #262.

// Q262_UndoRedoLogging represents the database schema/concept.
type Q262_UndoRedoLogging struct {
        tables map[string][]string
}

// NewQ262_UndoRedoLogging initializes the schema.
func NewQ262_UndoRedoLogging() *Q262_UndoRedoLogging {
        return &Q262_UndoRedoLogging{tables: make(map[string][]string)}
}
