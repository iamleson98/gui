// SPSC Ring Buffer — single-producer single-consumer bounded queue.
package concurrency

import "sync/atomic"

type SPSCRing[T any] struct {
	_    [64]byte
	buf  []T
	mask int32
	_    [56]byte
	head atomic.Int32
	_    [56]byte
	tail atomic.Int32
}

func NewSPSCRing[T any](capacity int) *SPSCRing[T] {
	if capacity <= 0 {
		capacity = 16
	}
	cap := 1
	for cap < capacity {
		cap <<= 1
	}
	return &SPSCRing[T]{buf: make([]T, cap), mask: int32(cap - 1)}
}

func (r *SPSCRing[T]) TryEnqueue(v T) bool {
	h := r.head.Load()
	t := r.tail.Load()
	if h-t >= r.mask+1 {
		return false
	}
	r.buf[h&r.mask] = v
	r.head.Store(h + 1)
	return true
}

func (r *SPSCRing[T]) TryDequeue() (v T, ok bool) {
	t := r.tail.Load()
	h := r.head.Load()
	if h == t {
		return
	}
	v = r.buf[t&r.mask]
	r.tail.Store(t + 1)
	return v, true
}

func (r *SPSCRing[T]) Len() int {
	return int(r.head.Load() - r.tail.Load())
}
