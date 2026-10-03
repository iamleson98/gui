package algorithms

import "testing"

func TestEditDistance(t *testing.T) {
	tests := []struct{ s1, s2 string; want int }{
		{"", "", 0},
		{"a", "", 1},
		{"", "a", 1},
		{"kitten", "sitting", 3},
		{"sunday", "saturday", 3},
		{"abc", "abc", 0},
		{"abc", "abd", 1},
		{"abc", "xyz", 3},
		{"intention", "execution", 5},
	}
	for _, tt := range tests {
		got := EditDistance(tt.s1, tt.s2)
		if got != tt.want {
			t.Errorf("EditDistance(%q, %q) = %d, want %d", tt.s1, tt.s2, got, tt.want)
		}
	}
}

func BenchmarkEditDistance(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		EditDistance("kitten", "sitting")
	}
}
