// Lock-Free MPSC Queue — multi-producer, single-consumer.
// Key concepts: CAS-free MPSC, atomic head swap, sentinel node.
//
// Design: tail always points to the "sentinel" (last consumed node).
// The actual value is in tail.next. After consuming, tail advances
// to tail.next (which becomes the new sentinel).
package concurrency

import "sync/atomic"

type mpscNode[T any] struct {
	value T
	next  atomic.Pointer[mpscNode[T]]
}

type MPSCQueue[T any] struct {
	head atomic.Pointer[mpscNode[T]] // producer end (most recently pushed)
	tail atomic.Pointer[mpscNode[T]] // consumer end (sentinel = last consumed)
}

func NewMPSCQueue[T any]() *MPSCQueue[T] {
	stub := &mpscNode[T]{}
	q := &MPSCQueue[T]{}
	q.head.Store(stub)
	q.tail.Store(stub)
	return q
}

// Enqueue adds v (multi-producer safe).
func (q *MPSCQueue[T]) Enqueue(v T) {
	n := &mpscNode[T]{value: v}
	old := q.head.Swap(n)
	old.next.Store(n)
}

// Dequeue removes and returns the head value. Returns ok=false if empty.
// MUST be called from a single consumer goroutine.
func (q *MPSCQueue[T]) Dequeue() (v T, ok bool) {
	tail := q.tail.Load()
	next := tail.next.Load()
	if next == nil {
		return // empty (tail is sentinel with no successor)
	}
	// Value is in next (tail is the sentinel/placeholder)
	v = next.value
	// Advance tail: next becomes the new sentinel
	q.tail.Store(next)
	return v, true
}

// Empty reports whether the queue is empty (consumer side).
func (q *MPSCQueue[T]) Empty() bool {
	return q.tail.Load().next.Load() == nil
}
