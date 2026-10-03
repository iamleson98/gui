// Skip List — probabilistic balanced structure.
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
