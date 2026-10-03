// LFU Cache — O(1) get/put using frequency buckets.
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
