// LRU Cache — O(1) get/put using hash map + doubly-linked list.
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
