package algorithms

import "testing"

func TestKMPFailureFunction(t *testing.T) {
	tests := []struct{ pattern string; fail []int }{
		{"abcabc", []int{0,0,0,1,2,3}},
		{"aaaa", []int{0,1,2,3}},
		{"ababab", []int{0,0,1,2,3,4}},
		{"abcxabc", []int{0,0,0,0,1,2,3}},
	}
	for _, tt := range tests {
		got := ComputeFailureFunction(tt.pattern)
		if len(got) != len(tt.fail) {
			t.Fatalf("fail length mismatch for %s", tt.pattern)
		}
		for i := range got {
			if got[i] != tt.fail[i] {
				t.Fatalf("fail[%s][%d] = %d, expected %d", tt.pattern, i, got[i], tt.fail[i])
			}
		}
	}
}

func TestKMPSearch(t *testing.T) {
	tests := []struct{ text, pattern string; expected []int }{
		{"hello world", "world", []int{6}},
		{"abababab", "ab", []int{0,2,4,6}},
		{"aaaaa", "aa", []int{0,1,2,3}},
		{"abcdef", "xyz", nil},
		{"abcdef", "abcdef", []int{0}},
		{"abc", "", []int{0}},
	}
	for _, tt := range tests {
		got := KMPSearch(tt.text, tt.pattern)
		if len(got) != len(tt.expected) {
			t.Fatalf("KMPSearch(%q, %q) = %v, expected %v", tt.text, tt.pattern, got, tt.expected)
		}
		for i := range got {
			if got[i] != tt.expected[i] {
				t.Fatalf("KMPSearch(%q, %q)[%d] = %d, expected %d", tt.text, tt.pattern, i, got[i], tt.expected[i])
			}
		}
	}
}

func BenchmarkKMP(b *testing.B) {
	text := ""
	for i := 0; i < 10000; i++ { text += "a" }
	text += "b"
	pattern := "aaaaab"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		KMPSearch(text, pattern)
	}
}
