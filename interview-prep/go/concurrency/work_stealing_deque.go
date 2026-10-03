// Chase-Lev Work-Stealing Deque.
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
        if d.top.CompareAndSwap(t, t+1) {
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
        if !d.top.CompareAndSwap(t, t+1) {
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
