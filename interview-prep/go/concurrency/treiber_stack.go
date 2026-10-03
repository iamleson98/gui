// Lock-Free Treiber Stack — uses CAS on a head pointer.
// Key concepts: CAS, ABA problem, atomic pointers, memory ordering.
package concurrency

import (
	"sync/atomic"
	"unsafe"
)

// node is a linked-list node holding a value and a next pointer.
type node[T any] struct {
	value T
	next  *node[T]
}

// TreiberStack is a lock-free LIFO stack.
type TreiberStack[T any] struct {
	head atomic.Pointer[node[T]]
	_    [56]byte // pad to 64 bytes to avoid false sharing
}

func NewTreiberStack[T any]() *TreiberStack[T] {
	return &TreiberStack[T]{}
}

// Push atomically prepends a new node.
func (s *TreiberStack[T]) Push(v T) {
	n := &node[T]{value: v}
	for {
		old := s.head.Load()
		n.next = old
		if s.head.CompareAndSwap(old, n) {
			return
		}
	}
}

// Pop atomically removes and returns the top node's value.
// Returns ok=false if the stack is empty.
func (s *TreiberStack[T]) Pop() (v T, ok bool) {
	for {
		old := s.head.Load()
		if old == nil {
			return // empty
		}
		next := old.next
		if s.head.CompareAndSwap(old, next) {
			return old.value, true
		}
	}
}

// Empty reports whether the stack has no elements.
func (s *TreiberStack[T]) Empty() bool {
	return s.head.Load() == nil
}

// _ avoids unused import warning.
var _ = unsafe.Sizeof(0)
