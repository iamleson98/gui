// Michael-Scott Lock-Free MPMC Queue — unbounded linked list queue.
package concurrency

import "sync/atomic"

type msNode[T any] struct {
	value T
	next  atomic.Pointer[msNode[T]]
}

type MSQueue[T any] struct {
	head atomic.Pointer[msNode[T]]
	tail atomic.Pointer[msNode[T]]
}

func NewMSQueue[T any]() *MSQueue[T] {
	dummy := &msNode[T]{}
	q := &MSQueue[T]{}
	q.head.Store(dummy)
	q.tail.Store(dummy)
	return q
}

func (q *MSQueue[T]) Enqueue(v T) {
	n := &msNode[T]{value: v}
	for {
		tail := q.tail.Load()
		next := tail.next.Load()
		if tail == q.tail.Load() {
			if next == nil {
				if tail.next.CompareAndSwap(nil, n) {
					q.tail.CompareAndSwap(tail, n)
					return
				}
			} else {
				q.tail.CompareAndSwap(tail, next)
			}
		}
	}
}

func (q *MSQueue[T]) Dequeue() (v T, ok bool) {
	for {
		head := q.head.Load()
		tail := q.tail.Load()
		next := head.next.Load()
		if head == q.head.Load() {
			if head == tail {
				if next == nil {
					return
				}
				q.tail.CompareAndSwap(tail, next)
			} else {
				val := next.value
				if q.head.CompareAndSwap(head, next) {
					return val, true
				}
			}
		}
	}
}

func (q *MSQueue[T]) Empty() bool {
	head := q.head.Load()
	tail := q.tail.Load()
	return head == tail && head.next.Load() == nil
}
