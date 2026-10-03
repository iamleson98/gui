// Concurrent Hash Map with Lock Striping.
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
