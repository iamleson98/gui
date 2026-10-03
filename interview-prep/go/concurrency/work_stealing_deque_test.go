package concurrency

import (
	"sync"
	"testing"
)

func TestWSDequeOwnerPushPop(t *testing.T) {
	d := NewWSDeque[int](16)
	for i := 0; i < 5; i++ {
		d.Push(i)
	}
	if d.Size() != 5 {
		t.Fatalf("expected size 5, got %d", d.Size())
	}
	v, ok := d.Pop()
	if !ok || v != 4 {
		t.Fatalf("expected 4, got %v ok=%v", v, ok)
	}
	v, _ = d.Pop()
	if v != 3 {
		t.Fatalf("expected 3, got %v", v)
	}
}

func TestWSDequeSteal(t *testing.T) {
	d := NewWSDeque[int](16)
	d.Push(1)
	d.Push(2)
	d.Push(3)
	v, ok := d.Steal()
	if !ok || v != 1 {
		t.Fatalf("expected steal 1, got %v ok=%v", v, ok)
	}
	v, _ = d.Steal()
	if v != 2 {
		t.Fatalf("expected steal 2, got %v", v)
	}
}

func TestWSDequeConcurrentSteal(t *testing.T) {
	d := NewWSDeque[int](1024)
	for i := 0; i < 1000; i++ {
		d.Push(i)
	}
	var wg sync.WaitGroup
	stolen := make(chan int, 100)
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				v, ok := d.Steal()
				if !ok {
					return
				}
				stolen <- v
			}
		}()
	}
	go func() {
		for {
			v, ok := d.Pop()
			if !ok {
				break
			}
			stolen <- v
		}
		wg.Wait()
		close(stolen)
	}()
	seen := make(map[int]bool)
	count := 0
	for v := range stolen {
		if seen[v] {
			t.Fatalf("duplicate %d", v)
		}
		seen[v] = true
		count++
	}
	if count != 1000 {
		t.Fatalf("expected 1000, got %d", count)
	}
}
