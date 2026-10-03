package algorithms

import "testing"

func TestLongestPalindrome(t *testing.T) {
	tests := []struct{ s, want string }{
		{"babad", "bab"},
		{"cbbd", "bb"},
		{"a", "a"},
		{"racecar", "racecar"},
		{"", ""},
	}
	for _, tt := range tests {
		got := LongestPalindrome(tt.s)
		if tt.s == "babad" {
			if got != "bab" && got != "aba" {
				t.Errorf("LongestPalindrome(%q) = %q, expected bab or aba", tt.s, got)
			}
			continue
		}
		if got != tt.want {
			t.Errorf("LongestPalindrome(%q) = %q, want %q", tt.s, got, tt.want)
		}
	}
}

func BenchmarkLongestPalindrome(b *testing.B) {
	s := ""
	for i := 0; i < 10000; i++ { s += "a" }
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		LongestPalindrome(s)
	}
}
