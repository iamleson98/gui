package datastructures

import "testing"

func TestTrieBasic(t *testing.T) {
	tr := NewTrie()
	tr.Insert("apple")
	if !tr.Search("apple") {
		t.Fatal("should find apple")
	}
	if tr.Search("app") {
		t.Fatal("should not find app")
	}
	if !tr.StartsWith("app") {
		t.Fatal("should have prefix app")
	}
}

func TestTrieDelete(t *testing.T) {
	tr := NewTrie()
	tr.Insert("apple")
	tr.Insert("app")
	if !tr.Delete("apple") {
		t.Fatal("delete apple failed")
	}
	if tr.Search("apple") {
		t.Fatal("apple should be deleted")
	}
	if !tr.Search("app") {
		t.Fatal("app should still exist")
	}
}

func TestTrieManyWords(t *testing.T) {
	words := []string{"the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog"}
	tr := NewTrie()
	for _, w := range words {
		tr.Insert(w)
	}
	for _, w := range words {
		if !tr.Search(w) {
			t.Fatalf("should find %s", w)
		}
	}
}

func BenchmarkTrieInsert(b *testing.B) {
	tr := NewTrie()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tr.Insert("benchmark")
	}
}

func BenchmarkTrieSearch(b *testing.B) {
	tr := NewTrie()
	words := []string{"apple", "application", "apricot", "banana", "cherry"}
	for _, w := range words {
		tr.Insert(w)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tr.Search("application")
	}
}
