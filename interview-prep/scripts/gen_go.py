#!/usr/bin/env python3
"""Generate all Go solution + test files for the interview-prep project."""
import os

BASE = "/home/z/my-project/interview-prep/go"

def write(path, content):
    full = os.path.join(BASE, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w") as f:
        f.write(content)
    print(f"  wrote {path}")

# ============================================================
# CONCURRENCY
# ============================================================

write("concurrency/michael_scott_queue.go", r'''// Michael-Scott Lock-Free MPMC Queue — unbounded linked list queue.
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
''')

write("concurrency/michael_scott_queue_test.go", r'''package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestMSQueueBasic(t *testing.T) {
	q := NewMSQueue[int]()
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	v, ok := q.Dequeue()
	if !ok || v != 1 {
		t.Fatalf("expected 1, got %v ok=%v", v, ok)
	}
	v, _ = q.Dequeue()
	if v != 2 {
		t.Fatalf("expected 2, got %v", v)
	}
	v, _ = q.Dequeue()
	if v != 3 {
		t.Fatalf("expected 3, got %v", v)
	}
	if _, ok := q.Dequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestMSQueueConcurrent(t *testing.T) {
	q := NewMSQueue[int]()
	var wg sync.WaitGroup
	N, M := 4, 2000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				q.Enqueue(id*M + j)
			}
		}(i)
	}
	var consumed int64
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < M; j++ {
				if _, ok := q.Dequeue(); ok {
					atomic.AddInt64(&consumed, 1)
				}
			}
		}()
	}
	wg.Wait()
	for {
		_, ok := q.Dequeue()
		if !ok {
			break
		}
		atomic.AddInt64(&consumed, 1)
	}
	total := int(consumed)
	expected := N * M
	if total != expected {
		t.Fatalf("expected %d, got %d", expected, total)
	}
}

func BenchmarkMSQueueEnqueue(b *testing.B) {
	q := NewMSQueue[int]()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		q.Enqueue(i)
	}
}

func BenchmarkMSQueueDequeue(b *testing.B) {
	q := NewMSQueue[int]()
	for i := 0; i < b.N; i++ {
		q.Enqueue(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		q.Dequeue()
	}
}

func BenchmarkMSQueueConcurrent(b *testing.B) {
	q := NewMSQueue[int]()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			q.Enqueue(i)
			q.Dequeue()
			i++
		}
	})
}
''')

write("concurrency/mpsc_queue.go", r'''// Lock-Free MPSC Queue — multi-producer, single-consumer.
package concurrency

import "sync/atomic"

type mpscNode[T any] struct {
	value T
	next  atomic.Pointer[mpscNode[T]]
}

type MPSCQueue[T any] struct {
	head atomic.Pointer[mpscNode[T]]
	stub mpscNode[T]
	tail atomic.Pointer[mpscNode[T]]
}

func NewMPSCQueue[T any]() *MPSCQueue[T] {
	q := &MPSCQueue[T]{}
	q.head.Store(&q.stub)
	q.tail.Store(&q.stub)
	return q
}

func (q *MPSCQueue[T]) Enqueue(v T) {
	n := &mpscNode[T]{value: v}
	old := q.head.Swap(n)
	old.next.Store(n)
}

func (q *MPSCQueue[T]) Dequeue() (v T, ok bool) {
	tail := q.tail.Load()
	next := tail.next.Load()
	if next == nil {
		return
	}
	q.tail.Store(next)
	if tail == &q.stub {
		return next.value, true
	}
	return tail.value, true
}

func (q *MPSCQueue[T]) Empty() bool {
	return q.tail.Load().next.Load() == nil
}
''')

write("concurrency/mpsc_queue_test.go", r'''package concurrency

import (
	"sync"
	"testing"
)

func TestMPSCQueueBasic(t *testing.T) {
	q := NewMPSCQueue[int]()
	if !q.Empty() {
		t.Fatal("new queue should be empty")
	}
	q.Enqueue(10)
	q.Enqueue(20)
	q.Enqueue(30)
	v, ok := q.Dequeue()
	if !ok || v != 10 {
		t.Fatalf("expected 10, got %v ok=%v", v, ok)
	}
	v, _ = q.Dequeue()
	if v != 20 {
		t.Fatalf("expected 20, got %v", v)
	}
	v, _ = q.Dequeue()
	if v != 30 {
		t.Fatalf("expected 30, got %v", v)
	}
	if _, ok := q.Dequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestMPSCQueueMultiProducer(t *testing.T) {
	q := NewMPSCQueue[int]()
	var wg sync.WaitGroup
	N, M := 8, 500
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				q.Enqueue(id*M + j)
			}
		}(i)
	}
	wg.Wait()
	seen := make(map[int]bool)
	count := 0
	for {
		v, ok := q.Dequeue()
		if !ok {
			break
		}
		if seen[v] {
			t.Fatalf("duplicate value %d", v)
		}
		seen[v] = true
		count++
	}
	expected := N * M
	if count != expected {
		t.Fatalf("expected %d, got %d", expected, count)
	}
}

func BenchmarkMPSCQueueEnqueue(b *testing.B) {
	q := NewMPSCQueue[int]()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			q.Enqueue(i)
			i++
		}
	})
}
''')

write("concurrency/spsc_ring.go", r'''// SPSC Ring Buffer — single-producer single-consumer bounded queue.
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
''')

write("concurrency/spsc_ring_test.go", r'''package concurrency

import "testing"

func TestSPSCRingBasic(t *testing.T) {
	r := NewSPSCRing[int](8)
	for i := 0; i < 8; i++ {
		if !r.TryEnqueue(i) {
			t.Fatalf("enqueue %d failed", i)
		}
	}
	if r.TryEnqueue(99) {
		t.Fatal("enqueue on full should fail")
	}
	for i := 0; i < 8; i++ {
		v, ok := r.TryDequeue()
		if !ok || v != i {
			t.Fatalf("dequeue %d: expected %d, got %v ok=%v", i, i, v, ok)
		}
	}
	if _, ok := r.TryDequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestSPSCRingInterleaved(t *testing.T) {
	r := NewSPSCRing[int](4)
	r.TryEnqueue(1)
	r.TryEnqueue(2)
	v, _ := r.TryDequeue()
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	r.TryEnqueue(3)
	r.TryEnqueue(4)
	r.TryEnqueue(5)
	for _, exp := range []int{2, 3, 4} {
		v, _ := r.TryDequeue()
		if v != exp {
			t.Fatalf("expected %d, got %v", exp, v)
		}
	}
}

func BenchmarkSPSCRingEnqueue(b *testing.B) {
	r := NewSPSCRing[int](1024)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		r.TryEnqueue(i)
		if r.Len() > 0 {
			r.TryDequeue()
		}
	}
}
''')

write("concurrency/ticket_spinlock.go", r'''// Ticket Spinlock — FIFO fair spinlock.
package concurrency

import (
	"runtime"
	"sync/atomic"
)

type TicketSpinlock struct {
	next atomic.Int64
	now  atomic.Int64
}

func (l *TicketSpinlock) Lock() {
	ticket := l.next.Add(1)
	for l.now.Load() != ticket {
		runtime.Gosched()
	}
}

func (l *TicketSpinlock) Unlock() {
	l.now.Add(1)
}

func (l *TicketSpinlock) TryLock() bool {
	ticket := l.next.Add(1)
	if l.now.Load() == ticket-1 {
		return true
	}
	l.next.Add(-1)
	return false
}
''')

write("concurrency/ticket_spinlock_test.go", r'''package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestTicketSpinlockBasic(t *testing.T) {
	var l TicketSpinlock
	l.Lock()
	if l.TryLock() {
		t.Fatal("TryLock should fail when locked")
	}
	l.Unlock()
	if !l.TryLock() {
		t.Fatal("TryLock should succeed when unlocked")
	}
	l.Unlock()
}

func TestTicketSpinlockMutualExclusion(t *testing.T) {
	var l TicketSpinlock
	var counter int64
	var wg sync.WaitGroup
	N := 8
	M := 10000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < M; j++ {
				l.Lock()
				atomic.AddInt64(&counter, 1)
				l.Unlock()
			}
		}()
	}
	wg.Wait()
	expected := int64(N * M)
	if counter != expected {
		t.Fatalf("expected %d, got %d", expected, counter)
	}
}

func BenchmarkTicketSpinlock(b *testing.B) {
	var l TicketSpinlock
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		l.Lock()
		l.Unlock()
	}
}
''')

write("concurrency/rwlock.go", r'''// Readers-Writer Lock (reader preference).
package concurrency

import "sync"

type RWLock struct {
	mu      sync.Mutex
	readers int
	writer  bool
	cond    *sync.Cond
}

func NewRWLock() *RWLock {
	l := &RWLock{}
	l.cond = sync.NewCond(&l.mu)
	return l
}

func (l *RWLock) RLock() {
	l.mu.Lock()
	for l.writer {
		l.cond.Wait()
	}
	l.readers++
	l.mu.Unlock()
}

func (l *RWLock) RUnlock() {
	l.mu.Lock()
	l.readers--
	if l.readers == 0 {
		l.cond.Broadcast()
	}
	l.mu.Unlock()
}

func (l *RWLock) Lock() {
	l.mu.Lock()
	for l.writer || l.readers > 0 {
		l.cond.Wait()
	}
	l.writer = true
	l.mu.Unlock()
}

func (l *RWLock) Unlock() {
	l.mu.Lock()
	l.writer = false
	l.cond.Broadcast()
	l.mu.Unlock()
}
''')

write("concurrency/rwlock_test.go", r'''package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestRWLockConcurrentReaders(t *testing.T) {
	l := NewRWLock()
	var wg sync.WaitGroup
	var active int64
	maxActive := int64(0)
	for i := 0; i < 16; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			l.RLock()
			cur := atomic.AddInt64(&active, 1)
			for {
				old := atomic.LoadInt64(&maxActive)
				if cur <= old || atomic.CompareAndSwapInt64(&maxActive, old, cur) {
					break
				}
			}
			atomic.AddInt64(&active, -1)
			l.RUnlock()
		}()
	}
	wg.Wait()
	if maxActive < 2 {
		t.Fatalf("expected multiple concurrent readers, max was %d", maxActive)
	}
}

func TestRWLockWriterExclusion(t *testing.T) {
	l := NewRWLock()
	var counter int64
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < 1000; j++ {
				l.Lock()
				counter++
				l.Unlock()
			}
		}()
	}
	wg.Wait()
	if counter != 4000 {
		t.Fatalf("expected 4000, got %d", counter)
	}
}

func BenchmarkRWLockRead(b *testing.B) {
	l := NewRWLock()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		l.RLock()
		l.RUnlock()
	}
}

func BenchmarkRWLockWrite(b *testing.B) {
	l := NewRWLock()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		l.Lock()
		l.Unlock()
	}
}
''')

write("concurrency/condvar.go", r'''// Condition Variable built on sync.Cond.
package concurrency

import "sync"

type CondVar struct {
	cond *sync.Cond
}

func NewCondVar() *CondVar {
	return &CondVar{cond: sync.NewCond(&sync.Mutex{})}
}

func (c *CondVar) Wait(pred func() bool) {
	c.cond.L.Lock()
	for !pred() {
		c.cond.Wait()
	}
	c.cond.L.Unlock()
}

func (c *CondVar) Signal() {
	c.cond.Signal()
}

func (c *CondVar) Broadcast() {
	c.cond.Broadcast()
}

func (c *CondVar) Lock()   { c.cond.L.Lock() }
func (c *CondVar) Unlock() { c.cond.L.Unlock() }
''')

write("concurrency/condvar_test.go", r'''package concurrency

import (
	"sync"
	"testing"
	"time"
)

func TestCondVarSignal(t *testing.T) {
	c := NewCondVar()
	ready := make(chan struct{})
	done := make(chan bool, 1)
	flag := false
	go func() {
		c.Lock()
		close(ready)
		c.Wait(func() bool { return flag })
		c.Unlock()
		done <- true
	}()
	<-ready
	time.Sleep(10 * time.Millisecond)
	c.Lock()
	flag = true
	c.Unlock()
	c.Signal()
	select {
	case <-done:
	case <-time.After(time.Second):
		t.Fatal("waiter did not wake up")
	}
}

func TestCondVarBroadcast(t *testing.T) {
	c := NewCondVar()
	var wg sync.WaitGroup
	flag := false
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c.Lock()
			c.Wait(func() bool { return flag })
			c.Unlock()
		}()
	}
	time.Sleep(20 * time.Millisecond)
	c.Lock()
	flag = true
	c.Unlock()
	c.Broadcast()
	wg.Wait()
}
''')

write("concurrency/work_stealing_deque.go", r'''// Chase-Lev Work-Stealing Deque.
package concurrency

import "sync/atomic"

type WSDeque[T any] struct {
	buf    []T
	mask   int32
	top    atomic.Int64
	_      [56]byte
	bottom atomic.Int64
}

func NewWSDeque[T any](capacity int) *WSDeque[T] {
	cap := 1
	for cap < capacity {
		cap <<= 1
	}
	return &WSDeque[T]{buf: make([]T, cap), mask: int32(cap - 1)}
}

func (d *WSDeque[T]) Push(v T) {
	b := d.bottom.Load()
	d.buf[int(b)&int(d.mask)] = v
	d.bottom.Store(b + 1)
}

func (d *WSDeque[T]) Pop() (v T, ok bool) {
	b := d.bottom.Load() - 1
	d.bottom.Store(b)
	t := d.top.Load()
	if t > b {
		d.bottom.Store(t)
		return
	}
	v = d.buf[int(b)&int(d.mask)]
	if t < b {
		return v, true
	}
	d.bottom.Store(t + 1)
	if atomic.CompareAndSwapInt64(&d.top, t, t+1) {
		return v, true
	}
	return
}

func (d *WSDeque[T]) Steal() (v T, ok bool) {
	t := d.top.Load()
	b := d.bottom.Load()
	if t >= b {
		return
	}
	v = d.buf[int(t)&int(d.mask)]
	if !atomic.CompareAndSwapInt64(&d.top, t, t+1) {
		return
	}
	return v, true
}

func (d *WSDeque[T]) Size() int {
	b := d.bottom.Load()
	t := d.top.Load()
	if b <= t {
		return 0
	}
	return int(b - t)
}
''')

write("concurrency/work_stealing_deque_test.go", r'''package concurrency

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
''')

write("concurrency/semaphore.go", r'''// Counting Semaphore via buffered channel.
package concurrency

type Semaphore struct {
	ch chan struct{}
}

func NewSemaphore(n int) *Semaphore {
	return &Semaphore{ch: make(chan struct{}, n)}
}

func (s *Semaphore) Acquire() {
	s.ch <- struct{}{}
}

func (s *Semaphore) Release() {
	<-s.ch
}

func (s *Semaphore) TryAcquire() bool {
	select {
	case s.ch <- struct{}{}:
		return true
	default:
		return false
	}
}
''')

write("concurrency/semaphore_test.go", r'''package concurrency

import (
	"sync"
	"testing"
)

func TestSemaphoreBasic(t *testing.T) {
	s := NewSemaphore(3)
	s.Acquire()
	s.Acquire()
	s.Acquire()
	if s.TryAcquire() {
		t.Fatal("should be full")
	}
	s.Release()
	if !s.TryAcquire() {
		t.Fatal("should have room after release")
	}
}

func TestSemaphoreConcurrency(t *testing.T) {
	s := NewSemaphore(4)
	var wg sync.WaitGroup
	var active int
	var maxActive int
	var mu sync.Mutex
	for i := 0; i < 20; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			s.Acquire()
			mu.Lock()
			active++
			if active > maxActive {
				maxActive = active
			}
			mu.Unlock()
			mu.Lock()
			active--
			mu.Unlock()
			s.Release()
		}()
	}
	wg.Wait()
	if maxActive > 4 {
		t.Fatalf("max active %d exceeded limit 4", maxActive)
	}
}

func BenchmarkSemaphore(b *testing.B) {
	s := NewSemaphore(1)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		s.Acquire()
		s.Release()
	}
}
''')

write("concurrency/concurrent_hashmap.go", r'''// Concurrent Hash Map with Lock Striping.
package concurrency

import (
	"hash/fnv"
	"sync"
)

type shard[K comparable, V any] struct {
	mu   sync.RWMutex
	data map[K]V
}

type ConcurrentMap[K comparable, V any] struct {
	shards []*shard[K, V]
	n      int
}

func NewConcurrentMap[K comparable, V any](shards int) *ConcurrentMap[K, V] {
	if shards <= 0 {
		shards = 32
	}
	m := &ConcurrentMap[K, V]{shards: make([]*shard[K, V], shards), n: shards}
	for i := range m.shards {
		m.shards[i] = &shard[K, V]{data: make(map[K]V)}
	}
	return m
}

func (m *ConcurrentMap[K, V]) idx(key K) int {
	h := fnv.New32a()
	s := any(key)
	if str, ok := s.(string); ok {
		h.Write([]byte(str))
	} else {
		h.Write([]byte(toString(s)))
	}
	return int(h.Sum32()) & (m.n - 1)
}

func toString(v interface{}) string {
	if v == nil {
		return ""
	}
	switch x := v.(type) {
	case string:
		return x
	case int:
		return string(rune(x))
	case int64:
		return string(rune(x))
	default:
		return ""
	}
}

func (m *ConcurrentMap[K, V]) Put(key K, val V) {
	s := m.shards[m.idx(key)]
	s.mu.Lock()
	s.data[key] = val
	s.mu.Unlock()
}

func (m *ConcurrentMap[K, V]) Get(key K) (V, bool) {
	s := m.shards[m.idx(key)]
	s.mu.RLock()
	v, ok := s.data[key]
	s.mu.RUnlock()
	return v, ok
}

func (m *ConcurrentMap[K, V]) Delete(key K) {
	s := m.shards[m.idx(key)]
	s.mu.Lock()
	delete(s.data, key)
	s.mu.Unlock()
}

func (m *ConcurrentMap[K, V]) Len() int {
	total := 0
	for _, s := range m.shards {
		s.mu.RLock()
		total += len(s.data)
		s.mu.RUnlock()
	}
	return total
}
''')

write("concurrency/concurrent_hashmap_test.go", r'''package concurrency

import (
	"fmt"
	"sync"
	"testing"
)

func TestConcurrentMapBasic(t *testing.T) {
	m := NewConcurrentMap[string, int](32)
	m.Put("a", 1)
	m.Put("b", 2)
	m.Put("c", 3)
	v, ok := m.Get("a")
	if !ok || v != 1 {
		t.Fatalf("expected a=1, got %v ok=%v", v, ok)
	}
	m.Delete("b")
	if _, ok := m.Get("b"); ok {
		t.Fatal("b should be deleted")
	}
	if m.Len() != 2 {
		t.Fatalf("expected len 2, got %d", m.Len())
	}
}

func TestConcurrentMapConcurrent(t *testing.T) {
	m := NewConcurrentMap[string, int](64)
	var wg sync.WaitGroup
	N := 8
	M := 1000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				key := fmt.Sprintf("k-%d-%d", id, j)
				m.Put(key, j)
			}
		}(i)
	}
	wg.Wait()
	if m.Len() != N*M {
		t.Fatalf("expected %d, got %d", N*M, m.Len())
	}
	for i := 0; i < N; i++ {
		for j := 0; j < M; j++ {
			key := fmt.Sprintf("k-%d-%d", i, j)
			v, ok := m.Get(key)
			if !ok || v != j {
				t.Fatalf("missing %s", key)
			}
		}
	}
}

func BenchmarkConcurrentMapPut(b *testing.B) {
	m := NewConcurrentMap[string, int](64)
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			m.Put(fmt.Sprintf("k-%d", i), i)
			i++
		}
	})
}

func BenchmarkConcurrentMapGet(b *testing.B) {
	m := NewConcurrentMap[string, int](64)
	for i := 0; i < 10000; i++ {
		m.Put(fmt.Sprintf("k-%d", i), i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			m.Get(fmt.Sprintf("k-%d", i%10000))
			i++
		}
	})
}
''')

# ============================================================
# DATA STRUCTURES
# ============================================================

write("datastructures/skip_list.go", r'''// Skip List — probabilistic balanced structure.
package datastructures

import "math/rand"

const maxLevel = 32

type skipNode[K comparable, V any] struct {
	key   K
	value V
	next  []*skipNode[K, V]
}

type SkipList[K comparable, V any] struct {
	head *skipNode[K, V]
	cmp  func(a, b K) int
	len  int
}

func NewSkipList[K comparable, V any](cmp func(a, b K) int) *SkipList[K, V] {
	if cmp == nil {
		cmp = func(a, b K) int {
			switch any(a).(type) {
			case string:
				sa, sb := any(a).(string), any(b).(string)
				if sa < sb { return -1 } else if sa > sb { return 1 }
				return 0
			case int:
				ia, ib := any(a).(int), any(b).(int)
				return ia - ib
			default:
				return 0
			}
		}
	}
	return &SkipList[K, V]{head: &skipNode[K, V]{next: make([]*skipNode[K, V], maxLevel)}, cmp: cmp}
}

func (s *SkipList[K, V]) randomLevel() int {
	lvl := 1
	for rand.Float64() < 0.5 && lvl < maxLevel {
		lvl++
	}
	return lvl
}

func (s *SkipList[K, V]) Insert(key K, value V) {
	update := make([]*skipNode[K, V], maxLevel)
	curr := s.head
	for i := maxLevel - 1; i >= 0; i-- {
		for curr.next[i] != nil && s.cmp(curr.next[i].key, key) < 0 {
			curr = curr.next[i]
		}
		update[i] = curr
	}
	curr = curr.next[0]
	if curr != nil && s.cmp(curr.key, key) == 0 {
		curr.value = value
		return
	}
	lvl := s.randomLevel()
	n := &skipNode[K, V]{key: key, value: value, next: make([]*skipNode[K, V], lvl)}
	for i := 0; i < lvl; i++ {
		n.next[i] = update[i].next[i]
		update[i].next[i] = n
	}
	s.len++
}

func (s *SkipList[K, V]) Search(key K) (V, bool) {
	curr := s.head
	for i := maxLevel - 1; i >= 0; i-- {
		for curr.next[i] != nil && s.cmp(curr.next[i].key, key) < 0 {
			curr = curr.next[i]
		}
	}
	curr = curr.next[0]
	if curr != nil && s.cmp(curr.key, key) == 0 {
		return curr.value, true
	}
	var zero V
	return zero, false
}

func (s *SkipList[K, V]) Delete(key K) bool {
	update := make([]*skipNode[K, V], maxLevel)
	curr := s.head
	for i := maxLevel - 1; i >= 0; i-- {
		for curr.next[i] != nil && s.cmp(curr.next[i].key, key) < 0 {
			curr = curr.next[i]
		}
		update[i] = curr
	}
	curr = curr.next[0]
	if curr == nil || s.cmp(curr.key, key) != 0 {
		return false
	}
	for i := 0; i < len(curr.next); i++ {
		update[i].next[i] = curr.next[i]
	}
	s.len--
	return true
}

func (s *SkipList[K, V]) Len() int { return s.len }
''')

write("datastructures/skip_list_test.go", r'''package datastructures

import (
	"fmt"
	"math/rand"
	"testing"
)

func TestSkipListBasic(t *testing.T) {
	sl := NewSkipList[int, string](nil)
	sl.Insert(1, "one")
	sl.Insert(2, "two")
	sl.Insert(3, "three")
	v, ok := sl.Search(2)
	if !ok || v != "two" {
		t.Fatalf("expected two, got %v ok=%v", v, ok)
	}
	if _, ok := sl.Search(99); ok {
		t.Fatal("should not find 99")
	}
	if sl.Len() != 3 {
		t.Fatalf("expected len 3, got %d", sl.Len())
	}
}

func TestSkipListDelete(t *testing.T) {
	sl := NewSkipList[int, int](nil)
	for i := 1; i <= 10; i++ {
		sl.Insert(i, i*10)
	}
	if !sl.Delete(5) {
		t.Fatal("delete 5 failed")
	}
	if _, ok := sl.Search(5); ok {
		t.Fatal("5 should be gone")
	}
	if sl.Len() != 9 {
		t.Fatalf("expected len 9, got %d", sl.Len())
	}
}

func TestSkipListRandom(t *testing.T) {
	rand.Seed(42)
	sl := NewSkipList[int, int](nil)
	N := 10000
	for i := 0; i < N; i++ {
		sl.Insert(rand.Intn(N), 1)
	}
	for i := 0; i < N; i++ {
		sl.Search(i)
	}
}

func TestSkipListStrings(t *testing.T) {
	sl := NewSkipList[string, int](nil)
	sl.Insert("banana", 1)
	sl.Insert("apple", 2)
	sl.Insert("cherry", 3)
	v, ok := sl.Search("apple")
	if !ok || v != 2 {
		t.Fatalf("expected apple=2, got %v", v)
	}
}

func BenchmarkSkipListInsert(b *testing.B) {
	sl := NewSkipList[int, int](nil)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		sl.Insert(i, i)
	}
}

func BenchmarkSkipListSearch(b *testing.B) {
	sl := NewSkipList[int, int](nil)
	for i := 0; i < 10000; i++ {
		sl.Insert(i, i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		sl.Search(i % 10000)
	}
}

func ExampleSkipList() {
	sl := NewSkipList[int, string](nil)
	sl.Insert(1, "hello")
	v, _ := sl.Search(1)
	fmt.Println(v)
	// Output: hello
}
''')

write("datastructures/lru_cache.go", r'''// LRU Cache — O(1) get/put using hash map + doubly-linked list.
package datastructures

import "container/list"

type lruEntry[K comparable, V any] struct {
	key   K
	value V
	elem  *list.Element
}

type LRUCache[K comparable, V any] struct {
	capacity int
	cache    map[K]*lruEntry[K, V]
	order    *list.List
}

func NewLRUCache[K comparable, V any](capacity int) *LRUCache[K, V] {
	if capacity <= 0 {
		capacity = 1
	}
	return &LRUCache[K, V]{capacity: capacity, cache: make(map[K]*lruEntry[K, V]), order: list.New()}
}

func (c *LRUCache[K, V]) Get(key K) (V, bool) {
	if e, ok := c.cache[key]; ok {
		c.order.MoveToFront(e.elem)
		return e.value, true
	}
	var zero V
	return zero, false
}

func (c *LRUCache[K, V]) Put(key K, value V) {
	if e, ok := c.cache[key]; ok {
		e.value = value
		c.order.MoveToFront(e.elem)
		return
	}
	elem := c.order.PushFront(key)
	e := &lruEntry[K, V]{key: key, value: value, elem: elem}
	c.cache[key] = e
	if len(c.cache) > c.capacity {
		oldest := c.order.Back()
		if oldest != nil {
			oldKey := oldest.Value.(K)
			c.order.Remove(oldest)
			delete(c.cache, oldKey)
		}
	}
}

func (c *LRUCache[K, V]) Len() int { return len(c.cache) }
''')

write("datastructures/lru_cache_test.go", r'''package datastructures

import "testing"

func TestLRUBasic(t *testing.T) {
	c := NewLRUCache[int, string](2)
	c.Put(1, "a")
	c.Put(2, "b")
	v, ok := c.Get(1)
	if !ok || v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
	c.Put(3, "c")
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should have been evicted")
	}
	v, _ = c.Get(1)
	if v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
}

func TestLRUUpdate(t *testing.T) {
	c := NewLRUCache[int, int](2)
	c.Put(1, 10)
	c.Put(1, 20)
	v, _ := c.Get(1)
	if v != 20 {
		t.Fatalf("expected 20, got %d", v)
	}
	if c.Len() != 1 {
		t.Fatalf("expected len 1, got %d", c.Len())
	}
}

func TestLRUEvictionOrder(t *testing.T) {
	c := NewLRUCache[int, int](3)
	c.Put(1, 1)
	c.Put(2, 2)
	c.Put(3, 3)
	c.Get(1)
	c.Put(4, 4)
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should be evicted")
	}
	for _, k := range []int{1, 3, 4} {
		if _, ok := c.Get(k); !ok {
			t.Fatalf("%d should be present", k)
		}
	}
}

func BenchmarkLRUPut(b *testing.B) {
	c := NewLRUCache[int, int](1000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		c.Put(i, i)
	}
}

func BenchmarkLRUGet(b *testing.B) {
	c := NewLRUCache[int, int](1000)
	for i := 0; i < 1000; i++ {
		c.Put(i, i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		c.Get(i % 1000)
	}
}
''')

write("datastructures/lfu_cache.go", r'''// LFU Cache — O(1) get/put using frequency buckets.
package datastructures

import "container/list"

type lfuEntry[K comparable, V any] struct {
	key   K
	value V
	freq  int
	elem  *list.Element
}

type LFUCache[K comparable, V any] struct {
	capacity int
	minFreq  int
	cache    map[K]*lfuEntry[K, V]
	freqs    map[int]*list.List
}

func NewLFUCache[K comparable, V any](capacity int) *LFUCache[K, V] {
	if capacity <= 0 {
		capacity = 1
	}
	return &LFUCache[K, V]{capacity: capacity, cache: make(map[K]*lfuEntry[K, V]), freqs: make(map[int]*list.List)}
}

func (c *LFUCache[K, V]) Get(key K) (V, bool) {
	e, ok := c.cache[key]
	if !ok {
		var zero V
		return zero, false
	}
	c.increment(e)
	return e.value, true
}

func (c *LFUCache[K, V]) Put(key K, value V) {
	if c.capacity == 0 {
		return
	}
	if e, ok := c.cache[key]; ok {
		e.value = value
		c.increment(e)
		return
	}
	if len(c.cache) >= c.capacity {
		c.evict()
	}
	e := &lfuEntry[K, V]{key: key, value: value, freq: 1}
	c.minFreq = 1
	l, ok := c.freqs[1]
	if !ok {
		l = list.New()
		c.freqs[1] = l
	}
	e.elem = l.PushFront(e)
	c.cache[key] = e
}

func (c *LFUCache[K, V]) increment(e *lfuEntry[K, V]) {
	oldList := c.freqs[e.freq]
	oldList.Remove(e.elem)
	if e.freq == c.minFreq && oldList.Len() == 0 {
		c.minFreq++
	}
	e.freq++
	l, ok := c.freqs[e.freq]
	if !ok {
		l = list.New()
		c.freqs[e.freq] = l
	}
	e.elem = l.PushFront(e)
}

func (c *LFUCache[K, V]) evict() {
	l := c.freqs[c.minFreq]
	if l == nil {
		return
	}
	back := l.Back()
	if back == nil {
		return
	}
	e := back.Value.(*lfuEntry[K, V])
	l.Remove(back)
	delete(c.cache, e.key)
}

func (c *LFUCache[K, V]) Len() int { return len(c.cache) }
''')

write("datastructures/lfu_cache_test.go", r'''package datastructures

import "testing"

func TestLFUBasic(t *testing.T) {
	c := NewLFUCache[int, string](2)
	c.Put(1, "a")
	c.Put(2, "b")
	c.Get(1)
	c.Get(2)
	c.Get(1)
	c.Put(3, "c")
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should be evicted")
	}
	v, _ := c.Get(1)
	if v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
}

func TestLFUEvictionByFrequency(t *testing.T) {
	c := NewLFUCache[int, int](3)
	c.Put(1, 1)
	c.Put(2, 2)
	c.Put(3, 3)
	c.Get(1)
	c.Get(1)
	c.Get(2)
	c.Put(4, 4)
	if _, ok := c.Get(3); ok {
		t.Fatal("3 should be evicted (lowest freq)")
	}
}
''')

write("datastructures/bloom_filter.go", r'''// Bloom Filter — probabilistic set membership.
package datastructures

import (
	"hash/fnv"
	"math"
)

type BloomFilter struct {
	bits []uint64
	m    int
	k    int
}

func NewBloomFilter(expectedN int, falsePositiveRate float64) *BloomFilter {
	if expectedN <= 0 {
		expectedN = 1000
	}
	if falsePositiveRate <= 0 || falsePositiveRate >= 1 {
		falsePositiveRate = 0.01
	}
	m := int(math.Ceil(-float64(expectedN) * math.Log(falsePositiveRate) / (math.Ln2 * math.Ln2)))
	k := int(math.Ceil(float64(m) / float64(expectedN) * math.Ln2))
	if k < 1 {
		k = 1
	}
	return &BloomFilter{bits: make([]uint64, (m+63)/64), m: m, k: k}
}

func (b *BloomFilter) hash(data []byte, seed int) int {
	h := fnv.New32a()
	h.Write(data)
	h.Write([]byte{byte(seed)})
	return int(h.Sum32()) % b.m
}

func (b *BloomFilter) Add(data []byte) {
	for i := 0; i < b.k; i++ {
		idx := b.hash(data, i)
		b.bits[idx/64] |= 1 << (idx % 64)
	}
}

func (b *BloomFilter) AddString(s string) { b.Add([]byte(s)) }

func (b *BloomFilter) Contains(data []byte) bool {
	for i := 0; i < b.k; i++ {
		idx := b.hash(data, i)
		if b.bits[idx/64]&(1<<(idx%64)) == 0 {
			return false
		}
	}
	return true
}

func (b *BloomFilter) ContainsString(s string) bool { return b.Contains([]byte(s)) }
''')

write("datastructures/bloom_filter_test.go", r'''package datastructures

import (
	"fmt"
	"testing"
)

func TestBloomFilterNoFalseNegatives(t *testing.T) {
	bf := NewBloomFilter(1000, 0.01)
	for i := 0; i < 1000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	for i := 0; i < 1000; i++ {
		if !bf.ContainsString(fmt.Sprintf("item-%d", i)) {
			t.Fatalf("false negative for item-%d", i)
		}
	}
}

func TestBloomFilterFalsePositiveRate(t *testing.T) {
	bf := NewBloomFilter(10000, 0.01)
	for i := 0; i < 10000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	fp := 0
	for i := 10000; i < 20000; i++ {
		if bf.ContainsString(fmt.Sprintf("item-%d", i)) {
			fp++
		}
	}
	if float64(fp)/10000 > 0.05 {
		t.Fatalf("false positive rate too high: fp=%d", fp)
	}
}

func BenchmarkBloomFilterAdd(b *testing.B) {
	bf := NewBloomFilter(100000, 0.01)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
}

func BenchmarkBloomFilterContains(b *testing.B) {
	bf := NewBloomFilter(100000, 0.01)
	for i := 0; i < 100000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bf.ContainsString(fmt.Sprintf("item-%d", i%100000))
	}
}
''')

write("datastructures/disjoint_set.go", r'''// Disjoint Set (Union-Find) with path compression + union by rank.
package datastructures

type DisjointSet struct {
	parent []int
	rank   []int
}

func NewDisjointSet(n int) *DisjointSet {
	d := &DisjointSet{parent: make([]int, n), rank: make([]int, n)}
	for i := range d.parent {
		d.parent[i] = i
	}
	return d
}

func (d *DisjointSet) Find(x int) int {
	if d.parent[x] != x {
		d.parent[x] = d.Find(d.parent[x])
	}
	return d.parent[x]
}

func (d *DisjointSet) Union(a, b int) bool {
	ra := d.Find(a)
	rb := d.Find(b)
	if ra == rb {
		return false
	}
	if d.rank[ra] < d.rank[rb] {
		ra, rb = rb, ra
	}
	d.parent[rb] = ra
	if d.rank[ra] == d.rank[rb] {
		d.rank[ra]++
	}
	return true
}

func (d *DisjointSet) Connected(a, b int) bool {
	return d.Find(a) == d.Find(b)
}
''')

write("datastructures/disjoint_set_test.go", r'''package datastructures

import "testing"

func TestDisjointSetBasic(t *testing.T) {
	d := NewDisjointSet(10)
	d.Union(0, 1)
	d.Union(2, 3)
	d.Union(1, 3)
	if !d.Connected(0, 2) {
		t.Fatal("0 and 2 should be connected")
	}
	if d.Connected(0, 4) {
		t.Fatal("0 and 4 should not be connected")
	}
}

func TestDisjointSetAllUnion(t *testing.T) {
	d := NewDisjointSet(100)
	for i := 0; i < 99; i++ {
		d.Union(i, i+1)
	}
	root := d.Find(0)
	for i := 1; i < 100; i++ {
		if d.Find(i) != root {
			t.Fatalf("node %d has different root", i)
		}
	}
}

func TestDisjointSetRedundantUnion(t *testing.T) {
	d := NewDisjointSet(5)
	if !d.Union(0, 1) {
		t.Fatal("first union should return true")
	}
	if d.Union(0, 1) {
		t.Fatal("second union should return false")
	}
}

func BenchmarkDisjointSet(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		d := NewDisjointSet(10000)
		for j := 0; j < 5000; j++ {
			d.Union(j, j+5000)
		}
		for j := 0; j < 10000; j++ {
			d.Find(j)
		}
	}
}
''')

write("datastructures/segment_tree.go", r'''// Segment Tree with Lazy Propagation.
package datastructures

type SegmentTree struct {
	n    int
	tree []int64
	lazy []int64
}

func NewSegmentTree(arr []int64) *SegmentTree {
	n := len(arr)
	st := &SegmentTree{n: n, tree: make([]int64, 4*n), lazy: make([]int64, 4*n)}
	if n > 0 {
		st.build(arr, 0, 0, n-1)
	}
	return st
}

func (st *SegmentTree) build(arr []int64, node, start, end int) {
	if start == end {
		st.tree[node] = arr[start]
		return
	}
	mid := (start + end) / 2
	st.build(arr, 2*node+1, start, mid)
	st.build(arr, 2*node+2, mid+1, end)
	st.tree[node] = st.tree[2*node+1] + st.tree[2*node+2]
}

func (st *SegmentTree) pushDown(node, start, end int) {
	if st.lazy[node] != 0 {
		st.tree[node] += int64(end-start+1) * st.lazy[node]
		if start != end {
			st.lazy[2*node+1] += st.lazy[node]
			st.lazy[2*node+2] += st.lazy[node]
		}
		st.lazy[node] = 0
	}
}

func (st *SegmentTree) UpdateRange(l, r int, val int64) {
	if st.n == 0 {
		return
	}
	st.updateRange(0, 0, st.n-1, l, r, val)
}

func (st *SegmentTree) updateRange(node, start, end, l, r int, val int64) {
	st.pushDown(node, start, end)
	if start > r || end < l {
		return
	}
	if l <= start && end <= r {
		st.lazy[node] += val
		st.pushDown(node, start, end)
		return
	}
	mid := (start + end) / 2
	st.updateRange(2*node+1, start, mid, l, r, val)
	st.updateRange(2*node+2, mid+1, end, l, r, val)
	st.tree[node] = st.tree[2*node+1] + st.tree[2*node+2]
}

func (st *SegmentTree) QueryRange(l, r int) int64 {
	if st.n == 0 {
		return 0
	}
	return st.queryRange(0, 0, st.n-1, l, r)
}

func (st *SegmentTree) queryRange(node, start, end, l, r int) int64 {
	st.pushDown(node, start, end)
	if start > r || end < l {
		return 0
	}
	if l <= start && end <= r {
		return st.tree[node]
	}
	mid := (start + end) / 2
	return st.queryRange(2*node+1, start, mid, l, r) + st.queryRange(2*node+2, mid+1, end, l, r)
}
''')

write("datastructures/segment_tree_test.go", r'''package datastructures

import "testing"

func TestSegmentTreeBasic(t *testing.T) {
	arr := []int64{1, 3, 5, 7, 9, 11}
	st := NewSegmentTree(arr)
	if v := st.QueryRange(0, 5); v != 36 {
		t.Fatalf("expected 36, got %d", v)
	}
	if v := st.QueryRange(1, 3); v != 15 {
		t.Fatalf("expected 15, got %d", v)
	}
}

func TestSegmentTreeRangeUpdate(t *testing.T) {
	arr := []int64{1, 2, 3, 4, 5}
	st := NewSegmentTree(arr)
	st.UpdateRange(0, 2, 10)
	if v := st.QueryRange(0, 2); v != 36 {
		t.Fatalf("expected 36, got %d", v)
	}
	if v := st.QueryRange(3, 4); v != 9 {
		t.Fatalf("expected 9, got %d", v)
	}
}

func BenchmarkSegmentTreeUpdate(b *testing.B) {
	arr := make([]int64, 100000)
	st := NewSegmentTree(arr)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		st.UpdateRange(0, 99999, 1)
	}
}

func BenchmarkSegmentTreeQuery(b *testing.B) {
	arr := make([]int64, 100000)
	st := NewSegmentTree(arr)
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		st.QueryRange(0, 99999)
	}
}
''')

write("datastructures/fenwick_tree.go", r'''// Fenwick Tree (Binary Indexed Tree).
package datastructures

type FenwickTree struct {
	tree []int64
	n    int
}

func NewFenwickTree(n int) *FenwickTree {
	return &FenwickTree{tree: make([]int64, n+1), n: n}
}

func NewFenwickTreeFrom(arr []int64) *FenwickTree {
	n := len(arr)
	ft := &FenwickTree{tree: make([]int64, n+1), n: n}
	for i := 0; i < n; i++ {
		ft.tree[i+1] = arr[i]
	}
	for i := 1; i <= n; i++ {
		p := i + (i & -i)
		if p <= n {
			ft.tree[p] += ft.tree[i]
		}
	}
	return ft
}

func (ft *FenwickTree) Update(i int, delta int64) {
	for i++; i <= ft.n; i += i & -i {
		ft.tree[i] += delta
	}
}

func (ft *FenwickTree) Query(i int) int64 {
	sum := int64(0)
	for i++; i > 0; i -= i & -i {
		sum += ft.tree[i]
	}
	return sum
}

func (ft *FenwickTree) QueryRange(l, r int) int64 {
	if l > r {
		return 0
	}
	return ft.Query(r) - ft.Query(l-1)
}
''')

write("datastructures/fenwick_tree_test.go", r'''package datastructures

import "testing"

func TestFenwickBasic(t *testing.T) {
	arr := []int64{1, 3, 5, 7, 9, 11}
	ft := NewFenwickTreeFrom(arr)
	if v := ft.Query(2); v != 9 {
		t.Fatalf("expected 9, got %d", v)
	}
	if v := ft.QueryRange(1, 3); v != 15 {
		t.Fatalf("expected 15, got %d", v)
	}
}

func TestFenwickUpdate(t *testing.T) {
	arr := []int64{10, 20, 30, 40, 50}
	ft := NewFenwickTreeFrom(arr)
	ft.Update(2, 5)
	if v := ft.Query(4); v != 155 {
		t.Fatalf("expected 155, got %d", v)
	}
}

func BenchmarkFenwickUpdate(b *testing.B) {
	ft := NewFenwickTree(100000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ft.Update(i%100000, 1)
	}
}

func BenchmarkFenwickQuery(b *testing.B) {
	ft := NewFenwickTree(100000)
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ft.Query(i % 100000)
	}
}
''')

write("datastructures/trie.go", r'''// Compressed Trie (Patricia Trie).
package datastructures

type trieNode struct {
	children map[byte]*trieNode
	isEnd    bool
}

type Trie struct {
	root *trieNode
}

func NewTrie() *Trie {
	return &Trie{root: &trieNode{children: make(map[byte]*trieNode)}}
}

func (t *Trie) Insert(word string) {
	node := t.root
	for i := 0; i < len(word); i++ {
		c := word[i]
		child, ok := node.children[c]
		if !ok {
			child = &trieNode{children: make(map[byte]*trieNode)}
			node.children[c] = child
		}
		node = child
	}
	node.isEnd = true
}

func (t *Trie) Search(word string) bool {
	node := t.root
	for i := 0; i < len(word); i++ {
		c := word[i]
		child, ok := node.children[c]
		if !ok {
			return false
		}
		node = child
	}
	return node.isEnd
}

func (t *Trie) StartsWith(prefix string) bool {
	node := t.root
	for i := 0; i < len(prefix); i++ {
		c := prefix[i]
		child, ok := node.children[c]
		if !ok {
			return false
		}
		node = child
	}
	return true
}

func (t *Trie) Delete(word string) bool {
	return t.delete(t.root, word, 0)
}

func (t *Trie) delete(node *trieNode, word string, depth int) bool {
	if depth == len(word) {
		if !node.isEnd {
			return false
		}
		node.isEnd = false
		return len(node.children) == 0
	}
	c := word[depth]
	child, ok := node.children[c]
	if !ok {
		return false
	}
	shouldDeleteChild := t.delete(child, word, depth+1)
	if shouldDeleteChild {
		delete(node.children, c)
		return len(node.children) == 0 && !node.isEnd
	}
	return false
}
''')

write("datastructures/trie_test.go", r'''package datastructures

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
''')

write("datastructures/btree.go", r'''// B-Tree — balanced search tree with multi-way nodes.
package datastructures

import "sort"

const btreeMinDegree = 4

type btreeNode struct {
	keys     []int
	children []*btreeNode
	leaf     bool
}

type BTree struct {
	root *btreeNode
	t    int
}

func NewBTree() *BTree {
	return &BTree{t: btreeMinDegree}
}

func (bt *BTree) Insert(key int) {
	if bt.root == nil {
		bt.root = &btreeNode{leaf: true}
	}
	r := bt.root
	if len(r.keys) >= 2*bt.t-1 {
		s := &btreeNode{leaf: false}
		s.children = append(s.children, r)
		bt.splitChild(s, 0)
		bt.root = s
	}
	bt.insertNonFull(bt.root, key)
}

func (bt *BTree) splitChild(parent *btreeNode, i int) {
	t := bt.t
	child := parent.children[i]
	mid := t - 1
	newNode := &btreeNode{leaf: child.leaf}
	newNode.keys = append(newNode.keys, child.keys[t:]...)
	if !child.leaf {
		newNode.children = append(newNode.children, child.children[t:]...)
		child.children = child.children[:t]
	}
	midKey := child.keys[mid]
	child.keys = child.keys[:mid]
	parent.keys = append(parent.keys, 0)
	copy(parent.keys[i+1:], parent.keys[i:])
	parent.keys[i] = midKey
	parent.children = append(parent.children, nil)
	copy(parent.children[i+2:], parent.children[i+1:])
	parent.children[i+1] = newNode
}

func (bt *BTree) insertNonFull(node *btreeNode, key int) {
	i := len(node.keys) - 1
	if node.leaf {
		node.keys = append(node.keys, 0)
		for i >= 0 && key < node.keys[i] {
			node.keys[i+1] = node.keys[i]
			i--
		}
		node.keys[i+1] = key
	} else {
		for i >= 0 && key < node.keys[i] {
			i--
		}
		i++
		if len(node.children[i].keys) >= 2*bt.t-1 {
			bt.splitChild(node, i)
			if key > node.keys[i] {
				i++
			}
		}
		bt.insertNonFull(node.children[i], key)
	}
}

func (bt *BTree) Search(key int) bool {
	return bt.search(bt.root, key)
}

func (bt *BTree) search(node *btreeNode, key int) bool {
	if node == nil {
		return false
	}
	i := sort.Search(len(node.keys), func(i int) bool { return node.keys[i] >= key })
	if i < len(node.keys) && node.keys[i] == key {
		return true
	}
	if node.leaf {
		return false
	}
	return bt.search(node.children[i], key)
}

func (bt *BTree) RangeQuery(lo, hi int) []int {
	result := []int{}
	bt.rangeQuery(bt.root, lo, hi, &result)
	return result
}

func (bt *BTree) rangeQuery(node *btreeNode, lo, hi int, result *[]int) {
	if node == nil {
		return
	}
	for i := 0; i < len(node.keys); i++ {
		if !node.leaf {
			bt.rangeQuery(node.children[i], lo, hi, result)
		}
		if node.keys[i] >= lo && node.keys[i] <= hi {
			*result = append(*result, node.keys[i])
		}
	}
	if !node.leaf {
		bt.rangeQuery(node.children[len(node.keys)], lo, hi, result)
	}
}
''')

write("datastructures/btree_test.go", r'''package datastructures

import (
	"math/rand"
	"testing"
)

func TestBTreeBasic(t *testing.T) {
	bt := NewBTree()
	for i := 1; i <= 100; i++ {
		bt.Insert(i)
	}
	for i := 1; i <= 100; i++ {
		if !bt.Search(i) {
			t.Fatalf("should find %d", i)
		}
	}
	if bt.Search(101) {
		t.Fatal("should not find 101")
	}
}

func TestBTreeRandom(t *testing.T) {
	rand.Seed(42)
	bt := NewBTree()
	values := rand.Perm(1000)
	for _, v := range values {
		bt.Insert(v + 1)
	}
	for _, v := range values {
		if !bt.Search(v + 1) {
			t.Fatalf("should find %d", v+1)
		}
	}
}

func TestBTreeRangeQuery(t *testing.T) {
	bt := NewBTree()
	for i := 1; i <= 100; i++ {
		bt.Insert(i)
	}
	result := bt.RangeQuery(10, 20)
	if len(result) != 11 {
		t.Fatalf("expected 11 results, got %d", len(result))
	}
}

func BenchmarkBTreeInsert(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bt := NewBTree()
		for j := 0; j < 1000; j++ {
			bt.Insert(j)
		}
	}
}

func BenchmarkBTreeSearch(b *testing.B) {
	bt := NewBTree()
	for i := 0; i < 10000; i++ {
		bt.Insert(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bt.Search(i % 10000)
	}
}
''')

write("datastructures/rbtree.go", r'''// Red-Black Tree — self-balancing BST.
package datastructures

import "fmt"

type color bool

const (
	red   color = true
	black color = false
)

type rbNode struct {
	key    int
	color  color
	left   *rbNode
	right  *rbNode
	parent *rbNode
}

type RBTree struct {
	root *rbNode
	nil  *rbNode
	size int
}

func NewRBTree() *RBTree {
	nil := &rbNode{color: black}
	return &RBTree{nil: nil, root: nil}
}

func (t *RBTree) Insert(key int) {
	z := &rbNode{key: key, color: red, left: t.nil, right: t.nil}
	var y *rbNode = t.nil
	x := t.root
	for x != t.nil {
		y = x
		if z.key < x.key {
			x = x.left
		} else {
			x = x.right
		}
	}
	z.parent = y
	if y == t.nil {
		t.root = z
	} else if z.key < y.key {
		y.left = z
	} else {
		y.right = z
	}
	t.size++
	t.insertFixup(z)
}

func (t *RBTree) leftRotate(x *rbNode) {
	y := x.right
	x.right = y.left
	if y.left != t.nil {
		y.left.parent = x
	}
	y.parent = x.parent
	if x.parent == t.nil {
		t.root = y
	} else if x == x.parent.left {
		x.parent.left = y
	} else {
		x.parent.right = y
	}
	y.left = x
	x.parent = y
}

func (t *RBTree) rightRotate(x *rbNode) {
	y := x.left
	x.left = y.right
	if y.right != t.nil {
		y.right.parent = x
	}
	y.parent = x.parent
	if x.parent == t.nil {
		t.root = y
	} else if x == x.parent.right {
		x.parent.right = y
	} else {
		x.parent.left = y
	}
	y.right = x
	x.parent = y
}

func (t *RBTree) insertFixup(z *rbNode) {
	for z.parent.color == red {
		if z.parent == z.parent.parent.left {
			y := z.parent.parent.right
			if y.color == red {
				z.parent.color = black
				y.color = black
				z.parent.parent.color = red
				z = z.parent.parent
			} else {
				if z == z.parent.right {
					z = z.parent
					t.leftRotate(z)
				}
				z.parent.color = black
				z.parent.parent.color = red
				t.rightRotate(z.parent.parent)
			}
		} else {
			y := z.parent.parent.left
			if y.color == red {
				z.parent.color = black
				y.color = black
				z.parent.parent.color = red
				z = z.parent.parent
			} else {
				if z == z.parent.left {
					z = z.parent
					t.rightRotate(z)
				}
				z.parent.color = black
				z.parent.parent.color = red
				t.leftRotate(z.parent.parent)
			}
		}
		if z == t.root {
			break
		}
	}
	t.root.color = black
}

func (t *RBTree) Search(key int) bool {
	node := t.root
	for node != t.nil {
		if key == node.key {
			return true
		}
		if key < node.key {
			node = node.left
		} else {
			node = node.right
		}
	}
	return false
}

func (t *RBTree) Inorder() []int {
	result := []int{}
	t.inorder(t.root, &result)
	return result
}

func (t *RBTree) inorder(node *rbNode, result *[]int) {
	if node == t.nil {
		return
	}
	t.inorder(node.left, result)
	*result = append(*result, node.key)
	t.inorder(node.right, result)
}

func (t *RBTree) Len() int { return t.size }

func (t *RBTree) VerifyInvariants() error {
	if t.root.color == red {
		return fmt.Errorf("root is red")
	}
	blackCount := -1
	return t.verify(t.root, 0, &blackCount)
}

func (t *RBTree) verify(node *rbNode, blackSoFar int, blackCount *int) error {
	if node == t.nil {
		if *blackCount == -1 {
			*blackCount = blackSoFar
		} else if blackSoFar != *blackCount {
			return fmt.Errorf("black height mismatch: %d vs %d", blackSoFar, *blackCount)
		}
		return nil
	}
	if node.color == red {
		if node.left.color == red || node.right.color == red {
			return fmt.Errorf("red node %d has red child", node.key)
		}
	}
	if node.color == black {
		blackSoFar++
	}
	if err := t.verify(node.left, blackSoFar, blackCount); err != nil {
		return err
	}
	return t.verify(node.right, blackSoFar, blackCount)
}
''')

write("datastructures/rbtree_test.go", r'''package datastructures

import (
	"math/rand"
	"testing"
)

func TestRBTreeBasic(t *testing.T) {
	tree := NewRBTree()
	for i := 1; i <= 100; i++ {
		tree.Insert(i)
	}
	for i := 1; i <= 100; i++ {
		if !tree.Search(i) {
			t.Fatalf("should find %d", i)
		}
	}
}

func TestRBTreeInvariants(t *testing.T) {
	rand.Seed(42)
	tree := NewRBTree()
	for i := 0; i < 1000; i++ {
		tree.Insert(rand.Intn(10000))
	}
	if err := tree.VerifyInvariants(); err != nil {
		t.Fatalf("invariant violation: %v", err)
	}
}

func TestRBTreeInorder(t *testing.T) {
	tree := NewRBTree()
	values := []int{5, 3, 7, 1, 4, 6, 8, 2}
	for _, v := range values {
		tree.Insert(v)
	}
	sorted := tree.Inorder()
	expected := []int{1, 2, 3, 4, 5, 6, 7, 8}
	for i, v := range sorted {
		if v != expected[i] {
			t.Fatalf("at index %d: expected %d, got %d", i, expected[i], v)
		}
	}
}

func BenchmarkRBTreeInsert(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tree := NewRBTree()
		for j := 0; j < 1000; j++ {
			tree.Insert(j)
		}
	}
}

func BenchmarkRBTreeSearch(b *testing.B) {
	tree := NewRBTree()
	for i := 0; i < 10000; i++ {
		tree.Insert(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tree.Search(i % 10000)
	}
}
''')

# ============================================================
# ALGORITHMS
# ============================================================

write("algorithms/edit_distance.go", r'''// Edit Distance (Levenshtein) — O(n*m) time, O(min(n,m)) space.
package algorithms

func EditDistance(s1, s2 string) int {
	n, m := len(s1), len(s2)
	if n > m {
		s1, s2 = s2, s1
		n, m = m, n
	}
	prev := make([]int, n+1)
	curr := make([]int, n+1)
	for i := 0; i <= n; i++ {
		prev[i] = i
	}
	for j := 1; j <= m; j++ {
		curr[0] = j
		for i := 1; i <= n; i++ {
			cost := 1
			if s1[i-1] == s2[j-1] {
				cost = 0
			}
			curr[i] = min3(prev[i]+1, curr[i-1]+1, prev[i-1]+cost)
		}
		prev, curr = curr, prev
	}
	return prev[n]
}

func min3(a, b, c int) int {
	m := a
	if b < m { m = b }
	if c < m { m = c }
	return m
}

func EditDistanceWithOps(s1, s2 string) []string {
	n, m := len(s1), len(s2)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, m+1)
		dp[i][0] = i
	}
	for j := 0; j <= m; j++ {
		dp[0][j] = j
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			cost := 1
			if s1[i-1] == s2[j-1] { cost = 0 }
			dp[i][j] = min3(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+cost)
		}
	}
	ops := []string{}
	i, j := n, m
	for i > 0 && j > 0 {
		cost := 1
		if s1[i-1] == s2[j-1] { cost = 0 }
		if dp[i][j] == dp[i-1][j-1]+cost && cost == 0 {
			ops = append([]string{"keep"}, ops...)
			i--; j--
		} else if dp[i][j] == dp[i-1][j-1]+1 {
			ops = append([]string{"sub"}, ops...)
			i--; j--
		} else if dp[i][j] == dp[i-1][j]+1 {
			ops = append([]string{"del"}, ops...)
			i--
		} else {
			ops = append([]string{"ins"}, ops...)
			j--
		}
	}
	for i > 0 { ops = append([]string{"del"}, ops...); i-- }
	for j > 0 { ops = append([]string{"ins"}, ops...); j-- }
	return ops
}
''')

write("algorithms/edit_distance_test.go", r'''package algorithms

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
''')

write("algorithms/lis.go", r'''// Longest Increasing Subsequence — O(n log n) using patience sorting.
package algorithms

func LISLength(nums []int) int {
	if len(nums) == 0 { return 0 }
	tails := []int{}
	for _, x := range nums {
		lo, hi := 0, len(tails)
		for lo < hi {
			mid := (lo + hi) / 2
			if tails[mid] < x { lo = mid + 1 } else { hi = mid }
		}
		if lo == len(tails) {
			tails = append(tails, x)
		} else {
			tails[lo] = x
		}
	}
	return len(tails)
}

func LIS(nums []int) []int {
	if len(nums) == 0 { return nil }
	tails := []int{}
	tailsIdx := []int{}
	prev := make([]int, len(nums))
	for i := range prev { prev[i] = -1 }
	for i, x := range nums {
		lo, hi := 0, len(tails)
		for lo < hi {
			mid := (lo + hi) / 2
			if tails[mid] < x { lo = mid + 1 } else { hi = mid }
		}
		if lo == len(tails) {
			tails = append(tails, x)
			tailsIdx = append(tailsIdx, i)
		} else {
			tails[lo] = x
			tailsIdx[lo] = i
		}
		if lo > 0 { prev[i] = tailsIdx[lo-1] }
	}
	result := []int{}
	k := tailsIdx[len(tailsIdx)-1]
	for k >= 0 {
		result = append([]int{nums[k]}, result...)
		k = prev[k]
	}
	return result
}
''')

write("algorithms/lis_test.go", r'''package algorithms

import (
	"math/rand"
	"sort"
	"testing"
)

func TestLISLength(t *testing.T) {
	tests := []struct{ nums []int; want int }{
		{nil, 0},
		{[]int{1}, 1},
		{[]int{1,2,3,4,5}, 5},
		{[]int{5,4,3,2,1}, 1},
		{[]int{10,9,2,5,3,7,101,18}, 4},
		{[]int{0,1,0,3,2,3}, 4},
		{[]int{7,7,7,7,7,7,7}, 1},
	}
	for _, tt := range tests {
		got := LISLength(tt.nums)
		if got != tt.want {
			t.Errorf("LISLength(%v) = %d, want %d", tt.nums, got, tt.want)
		}
	}
}

func TestLISReconstruct(t *testing.T) {
	nums := []int{10, 9, 2, 5, 3, 7, 101, 18}
	result := LIS(nums)
	if len(result) != 4 {
		t.Fatalf("expected length 4, got %d", len(result))
	}
	for i := 1; i < len(result); i++ {
		if result[i] <= result[i-1] {
			t.Fatalf("not increasing: %v", result)
		}
	}
}

func TestLISRandom(t *testing.T) {
	rand.Seed(42)
	for trial := 0; trial < 100; trial++ {
		n := rand.Intn(100) + 1
		nums := make([]int, n)
		for i := range nums { nums[i] = rand.Intn(1000) }
		result := LIS(nums)
		j := 0
		for _, x := range nums {
			if j < len(result) && x == result[j] { j++ }
		}
		if j != len(result) {
			t.Fatalf("trial %d: result is not a subsequence", trial)
		}
	}
}

func BenchmarkLIS(b *testing.B) {
	nums := make([]int, 10000)
	for i := range nums { nums[i] = i }
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		LISLength(nums)
	}
}
''')

write("algorithms/dijkstra.go", r'''// Dijkstra's Algorithm — single-source shortest path.
package algorithms

import "container/heap"

type edge struct {
	to   int
	cost int64
}

type Graph struct {
	n   int
	adj [][]edge
}

func NewGraph(n int) *Graph {
	return &Graph{n: n, adj: make([][]edge, n)}
}

func (g *Graph) AddEdge(from, to int, cost int64) {
	g.adj[from] = append(g.adj[from], edge{to, cost})
}

type pqItem struct {
	node int
	dist int64
	idx  int
}

type priorityQueue []*pqItem

func (pq priorityQueue) Len() int            { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool  { return pq[i].dist < pq[j].dist }
func (pq priorityQueue) Swap(i, j int)       { pq[i], pq[j] = pq[j], pq[i]; pq[i].idx = i; pq[j].idx = j }
func (pq *priorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*pqItem)
	item.idx = n
	*pq = append(*pq, item)
}
func (pq *priorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[:n-1]
	return item
}

func Dijkstra(g *Graph, source int) []int64 {
	dist := make([]int64, g.n)
	for i := range dist { dist[i] = -1 }
	dist[source] = 0
	pq := &priorityQueue{}
	heap.Init(pq)
	heap.Push(pq, &pqItem{node: source, dist: 0})
	visited := make([]bool, g.n)
	for pq.Len() > 0 {
		item := heap.Pop(pq).(*pqItem)
		u := item.node
		if visited[u] { continue }
		visited[u] = true
		for _, e := range g.adj[u] {
			v := e.to
			newDist := dist[u] + e.cost
			if dist[v] == -1 || newDist < dist[v] {
				dist[v] = newDist
				heap.Push(pq, &pqItem{node: v, dist: newDist})
			}
		}
	}
	return dist
}
''')

write("algorithms/dijkstra_test.go", r'''package algorithms

import "testing"

func TestDijkstraBasic(t *testing.T) {
	g := NewGraph(5)
	g.AddEdge(0, 1, 10)
	g.AddEdge(0, 4, 5)
	g.AddEdge(1, 2, 1)
	g.AddEdge(1, 4, 2)
	g.AddEdge(2, 3, 4)
	g.AddEdge(3, 0, 7)
	g.AddEdge(4, 1, 3)
	g.AddEdge(4, 2, 9)
	g.AddEdge(4, 3, 2)
	dist := Dijkstra(g, 0)
	expected := []int64{0, 8, 9, 7, 5}
	for i, d := range dist {
		if d != expected[i] {
			t.Fatalf("dist[%d] = %d, expected %d", i, d, expected[i])
		}
	}
}

func TestDijkstraUnreachable(t *testing.T) {
	g := NewGraph(4)
	g.AddEdge(0, 1, 1)
	g.AddEdge(1, 2, 1)
	dist := Dijkstra(g, 0)
	if dist[3] != -1 {
		t.Fatalf("dist[3] should be -1, got %d", dist[3])
	}
}

func BenchmarkDijkstra(b *testing.B) {
	n := 1000
	g := NewGraph(n)
	for i := 0; i < n; i++ {
		for j := 1; j <= 5; j++ {
			g.AddEdge(i, (i+j)%n, int64(j))
		}
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Dijkstra(g, 0)
	}
}
''')

write("algorithms/kmp.go", r'''// KMP String Matching — O(n+m).
package algorithms

func ComputeFailureFunction(pattern string) []int {
	n := len(pattern)
	if n == 0 { return nil }
	fail := make([]int, n)
	j := 0
	for i := 1; i < n; {
		if pattern[i] == pattern[j] {
			j++
			fail[i] = j
			i++
		} else if j > 0 {
			j = fail[j-1]
		} else {
			fail[i] = 0
			i++
		}
	}
	return fail
}

func KMPSearch(text, pattern string) []int {
	if len(pattern) == 0 { return []int{0} }
	if len(pattern) > len(text) { return nil }
	fail := ComputeFailureFunction(pattern)
	result := []int{}
	j := 0
	for i := 0; i < len(text); {
		if text[i] == pattern[j] {
			i++
			j++
			if j == len(pattern) {
				result = append(result, i-j)
				j = fail[j-1]
			}
		} else if j > 0 {
			j = fail[j-1]
		} else {
			i++
		}
	}
	return result
}
''')

write("algorithms/kmp_test.go", r'''package algorithms

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
''')

write("algorithms/convex_hull.go", r'''// Convex Hull — Andrew's monotone chain in O(n log n).
package algorithms

import "sort"

type Point struct {
	X, Y float64
}

func Cross(O, A, B Point) float64 {
	return (A.X-O.X)*(B.Y-O.Y) - (A.Y-O.Y)*(B.X-O.X)
}

func ConvexHull(points []Point) []Point {
	n := len(points)
	if n <= 2 { return points }
	sorted := make([]Point, n)
	copy(sorted, points)
	sort.Slice(sorted, func(i, j int) bool {
		if sorted[i].X != sorted[j].X { return sorted[i].X < sorted[j].X }
		return sorted[i].Y < sorted[j].Y
	})
	lower := []Point{}
	for _, p := range sorted {
		for len(lower) >= 2 && Cross(lower[len(lower)-2], lower[len(lower)-1], p) <= 0 {
			lower = lower[:len(lower)-1]
		}
		lower = append(lower, p)
	}
	upper := []Point{}
	for i := n - 1; i >= 0; i-- {
		p := sorted[i]
		for len(upper) >= 2 && Cross(upper[len(upper)-2], upper[len(upper)-1], p) <= 0 {
			upper = upper[:len(upper)-1]
		}
		upper = append(upper, p)
	}
	return append(lower[:len(lower)-1], upper[:len(upper)-1]...)
}
''')

write("algorithms/convex_hull_test.go", r'''package algorithms

import (
	"math"
	"testing"
)

func pointsEqual(a, b Point) bool {
	return math.Abs(a.X-b.X) < 1e-9 && math.Abs(a.Y-b.Y) < 1e-9
}

func TestConvexHullBasic(t *testing.T) {
	points := []Point{
		{0, 0}, {1, 0}, {0, 1}, {1, 1},
		{0.5, 0.5},
	}
	hull := ConvexHull(points)
	if len(hull) != 4 {
		t.Fatalf("expected 4 hull points, got %d: %v", len(hull), hull)
	}
}

func TestConvexHullCollinear(t *testing.T) {
	points := []Point{
		{0, 0}, {1, 0}, {2, 0}, {3, 0},
	}
	hull := ConvexHull(points)
	if len(hull) < 2 {
		t.Fatalf("expected at least 2 hull points, got %d", len(hull))
	}
}

func BenchmarkConvexHull(b *testing.B) {
	points := make([]Point, 10000)
	for i := range points {
		points[i] = Point{X: float64(i % 100), Y: float64(i / 100)}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ConvexHull(points)
	}
}
''')

write("algorithms/knapsack.go", r'''// 0/1 Knapsack — classic DP.
package algorithms

type Item struct {
	Weight int
	Value  int
}

func Knapsack01(items []Item, capacity int) int {
	n := len(items)
	dp := make([]int, capacity+1)
	for i := 0; i < n; i++ {
		for w := capacity; w >= items[i].Weight; w-- {
			if dp[w-items[i].Weight]+items[i].Value > dp[w] {
				dp[w] = dp[w-items[i].Weight] + items[i].Value
			}
		}
	}
	return dp[capacity]
}

func Knapsack01WithItems(items []Item, capacity int) []int {
	n := len(items)
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, capacity+1)
	}
	for i := 1; i <= n; i++ {
		for w := 0; w <= capacity; w++ {
			dp[i][w] = dp[i-1][w]
			if items[i-1].Weight <= w {
				if dp[i-1][w-items[i-1].Weight]+items[i-1].Value > dp[i][w] {
					dp[i][w] = dp[i-1][w-items[i-1].Weight] + items[i-1].Value
				}
			}
		}
	}
	result := []int{}
	w := capacity
	for i := n; i > 0; i-- {
		if dp[i][w] != dp[i-1][w] {
			result = append([]int{i - 1}, result...)
			w -= items[i-1].Weight
		}
	}
	return result
}
''')

write("algorithms/knapsack_test.go", r'''package algorithms

import "testing"

func TestKnapsack01(t *testing.T) {
	items := []Item{
		{Weight: 2, Value: 3},
		{Weight: 3, Value: 4},
		{Weight: 4, Value: 5},
		{Weight: 5, Value: 6},
	}
	result := Knapsack01(items, 5)
	if result != 7 {
		t.Fatalf("expected 7, got %d", result)
	}
}

func TestKnapsackEmpty(t *testing.T) {
	if Knapsack01(nil, 10) != 0 {
		t.Fatal("empty knapsack should be 0")
	}
}

func TestKnapsackNoFit(t *testing.T) {
	items := []Item{{Weight: 100, Value: 1000}}
	if Knapsack01(items, 10) != 0 {
		t.Fatal("should be 0 when nothing fits")
	}
}

func BenchmarkKnapsack(b *testing.B) {
	items := make([]Item, 100)
	for i := range items {
		items[i] = Item{Weight: i + 1, Value: i*2 + 1}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Knapsack01(items, 5000)
	}
}
''')

write("algorithms/max_flow_dinic.go", r'''// Max Flow — Dinic's algorithm.
package algorithms

type flowEdge struct {
	to   int
	cap  int64
	rev  int
}

type MaxFlow struct {
	n     int
	graph [][]flowEdge
}

func NewMaxFlow(n int) *MaxFlow {
	return &MaxFlow{n: n, graph: make([][]flowEdge, n)}
}

func (mf *MaxFlow) AddEdge(from, to int, cap int64) {
	mf.graph[from] = append(mf.graph[from], flowEdge{to: to, cap: cap, rev: len(mf.graph[to])})
	mf.graph[to] = append(mf.graph[to], flowEdge{to: from, cap: 0, rev: len(mf.graph[from]) - 1})
}

func (mf *MaxFlow) bfs(s, t int, level []int) bool {
	for i := range level { level[i] = -1 }
	level[s] = 0
	queue := []int{s}
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for _, e := range mf.graph[u] {
			if e.cap > 0 && level[e.to] < 0 {
				level[e.to] = level[u] + 1
				queue = append(queue, e.to)
			}
		}
	}
	return level[t] >= 0
}

func (mf *MaxFlow) dfs(u, t int, f int64, level []int, iter []int) int64 {
	if u == t { return f }
	for ; iter[u] < len(mf.graph[u]); iter[u]++ {
		e := &mf.graph[u][iter[u]]
		if e.cap > 0 && level[e.to] == level[u]+1 {
			d := mf.dfs(e.to, t, min64(f, e.cap), level, iter)
			if d > 0 {
				e.cap -= d
				mf.graph[e.to][e.rev].cap += d
				return d
			}
		}
	}
	return 0
}

func (mf *MaxFlow) MaxFlow(s, t int) int64 {
	flow := int64(0)
	level := make([]int, mf.n)
	iter := make([]int, mf.n)
	const INF int64 = 1 << 60
	for mf.bfs(s, t, level) {
		for i := range iter { iter[i] = 0 }
		for {
			f := mf.dfs(s, t, INF, level, iter)
			if f == 0 { break }
			flow += f
		}
	}
	return flow
}

func min64(a, b int64) int64 {
	if a < b { return a }
	return b
}
''')

write("algorithms/max_flow_dinic_test.go", r'''package algorithms

import "testing"

func TestMaxFlowBasic(t *testing.T) {
	mf := NewMaxFlow(4)
	mf.AddEdge(0, 1, 3)
	mf.AddEdge(0, 2, 2)
	mf.AddEdge(1, 2, 1)
	mf.AddEdge(1, 3, 2)
	mf.AddEdge(2, 3, 3)
	flow := mf.MaxFlow(0, 3)
	if flow != 4 {
		t.Fatalf("expected 4, got %d", flow)
	}
}

func TestMaxFlowSinglePath(t *testing.T) {
	mf := NewMaxFlow(3)
	mf.AddEdge(0, 1, 5)
	mf.AddEdge(1, 2, 3)
	flow := mf.MaxFlow(0, 2)
	if flow != 3 {
		t.Fatalf("expected 3, got %d", flow)
	}
}

func TestMaxFlowNoPath(t *testing.T) {
	mf := NewMaxFlow(3)
	mf.AddEdge(0, 1, 5)
	flow := mf.MaxFlow(0, 2)
	if flow != 0 {
		t.Fatalf("expected 0, got %d", flow)
	}
}

func BenchmarkMaxFlow(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		mf := NewMaxFlow(100)
		for j := 0; j < 99; j++ {
			mf.AddEdge(j, j+1, 10)
		}
		mf.MaxFlow(0, 99)
	}
}
''')

write("algorithms/floyd_warshall.go", r'''// Floyd-Warshall — all-pairs shortest paths in O(V^3).
package algorithms

const FW_INF int64 = 1 << 60

func FloydWarshall(dist [][]int64) [][]int64 {
	n := len(dist)
	result := make([][]int64, n)
	for i := range result {
		result[i] = make([]int64, n)
		copy(result[i], dist[i])
	}
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				if result[i][k] != FW_INF && result[k][j] != FW_INF {
					if result[i][k]+result[k][j] < result[i][j] {
						result[i][j] = result[i][k] + result[k][j]
					}
				}
			}
		}
	}
	return result
}

func HasNegativeCycle(dist [][]int64) bool {
	n := len(dist)
	result := FloydWarshall(dist)
	for i := 0; i < n; i++ {
		if result[i][i] < 0 {
			return true
		}
	}
	return false
}
''')

write("algorithms/floyd_warshall_test.go", r'''package algorithms

import "testing"

func TestFloydWarshall(t *testing.T) {
	INF := FW_INF
	dist := [][]int64{
		{0, 3, INF, 5},
		{2, 0, INF, 4},
		{INF, 1, 0, INF},
		{INF, INF, 2, 0},
	}
	result := FloydWarshall(dist)
	expected := [][]int64{
		{0, 3, 7, 5},
		{2, 0, 6, 4},
		{3, 1, 0, 5},
		{5, 3, 2, 0},
	}
	for i := range result {
		for j := range result[i] {
			if result[i][j] != expected[i][j] {
				t.Fatalf("result[%d][%d] = %d, expected %d", i, j, result[i][j], expected[i][j])
			}
		}
	}
}

func TestFloydWarshallNegativeCycle(t *testing.T) {
	INF := FW_INF
	dist := [][]int64{
		{0, 1, INF},
		{INF, 0, -1},
		{-1, INF, 0},
	}
	if !HasNegativeCycle(dist) {
		t.Fatal("should detect negative cycle")
	}
}

func BenchmarkFloydWarshall(b *testing.B) {
	n := 100
	dist := make([][]int64, n)
	for i := range dist {
		dist[i] = make([]int64, n)
		for j := range dist[i] {
			if i == j { dist[i][j] = 0 } else { dist[i][j] = FW_INF }
		}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		FloydWarshall(dist)
	}
}
''')

write("algorithms/quickselect.go", r'''// Quickselect — find k-th smallest in expected O(n).
package algorithms

import "math/rand"

func QuickSelect(nums []int, k int) int {
	if k < 0 || k >= len(nums) {
		panic("k out of range")
	}
	return quickSelect(nums, 0, len(nums)-1, k)
}

func quickSelect(nums []int, lo, hi, k int) int {
	if lo == hi { return nums[lo] }
	pivotIdx := lo + rand.Intn(hi-lo+1)
	pivotIdx = partition(nums, lo, hi, pivotIdx)
	if k == pivotIdx { return nums[k] }
	if k < pivotIdx { return quickSelect(nums, lo, pivotIdx-1, k) }
	return quickSelect(nums, pivotIdx+1, hi, k)
}

func partition(nums []int, lo, hi, pivotIdx int) int {
	pivot := nums[pivotIdx]
	nums[pivotIdx], nums[hi] = nums[hi], nums[pivotIdx]
	store := lo
	for i := lo; i < hi; i++ {
		if nums[i] < pivot {
			nums[store], nums[i] = nums[i], nums[store]
			store++
		}
	}
	nums[store], nums[hi] = nums[hi], nums[store]
	return store
}
''')

write("algorithms/quickselect_test.go", r'''package algorithms

import (
	"math/rand"
	"sort"
	"testing"
)

func TestQuickSelect(t *testing.T) {
	nums := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)
	for k := 0; k < len(nums); k++ {
		arr := make([]int, len(nums))
		copy(arr, nums)
		got := QuickSelect(arr, k)
		if got != sorted[k] {
			t.Fatalf("QuickSelect(k=%d) = %d, expected %d", k, got, sorted[k])
		}
	}
}

func TestQuickSelectRandom(t *testing.T) {
	rand.Seed(42)
	for trial := 0; trial < 100; trial++ {
		n := rand.Intn(100) + 1
		nums := rand.Perm(n)
		sorted := make([]int, n)
		copy(sorted, nums)
		sort.Ints(sorted)
		k := rand.Intn(n)
		arr := make([]int, n)
		copy(arr, nums)
		got := QuickSelect(arr, k)
		if got != sorted[k] {
			t.Fatalf("trial %d: QuickSelect(k=%d) = %d, expected %d", trial, k, got, sorted[k])
		}
	}
}

func BenchmarkQuickSelect(b *testing.B) {
	nums := rand.Perm(100000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		arr := make([]int, len(nums))
		copy(arr, nums)
		QuickSelect(arr, 50000)
	}
}
''')

write("algorithms/manacher.go", r'''// Manacher's Algorithm — longest palindromic substring in O(n).
package algorithms

func LongestPalindrome(s string) string {
	if len(s) == 0 { return "" }
	t := "^"
	for i := 0; i < len(s); i++ {
		t += "#" + string(s[i])
	}
	t += "#$"
	n := len(t)
	p := make([]int, n)
	c, r := 0, 0
	maxLen, center := 0, 0
	for i := 1; i < n-1; i++ {
		mirror := 2*c - i
		if r > i {
			p[i] = minInt(r-i, p[mirror])
		}
		for t[i+p[i]+1] == t[i-p[i]-1] {
			p[i]++
		}
		if i+p[i] > r {
			c = i
			r = i + p[i]
		}
		if p[i] > maxLen {
			maxLen = p[i]
			center = i
		}
	}
	start := (center - maxLen) / 2
	return s[start : start+maxLen]
}

func minInt(a, b int) int {
	if a < b { return a }
	return b
}
''')

write("algorithms/manacher_test.go", r'''package algorithms

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
''')

write("README.md", r'''# Interview Prep — Go Solutions

This directory contains Go solutions, tests, and benchmarks for senior-level
interview questions.

## Structure

- `concurrency/` — Lock-free data structures, atomics, sync primitives
- `datastructures/` — Skip lists, B-trees, caches, tries, etc.
- `algorithms/` — DP, graph algorithms, string matching, etc.

## Running

```bash
go test ./...
go test -bench=. -benchmem ./...
```
''')

print("\nAll Go files generated!")
