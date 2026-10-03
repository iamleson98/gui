// Question #231: Surrogate vs Natural Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: surrogate key, natural key, stability, joins
// Description: Choose between surrogate and natural keys, weighing stability, size, and join performance.
package sql


// Surrogate vs Natural Keys
// Implements a database design pattern for question #231.

// Q231_SurrogateVsNaturalKeys represents the database schema/concept.
type Q231_SurrogateVsNaturalKeys struct {
        tables map[string][]string
}

// NewQ231_SurrogateVsNaturalKeys initializes the schema.
func NewQ231_SurrogateVsNaturalKeys() *Q231_SurrogateVsNaturalKeys {
        return &Q231_SurrogateVsNaturalKeys{tables: make(map[string][]string)}
}
