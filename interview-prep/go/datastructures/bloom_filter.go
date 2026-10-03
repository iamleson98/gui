// Bloom Filter — probabilistic set membership.
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
        // Use double hashing: h_i = h1 + i * h2 mod m
        h1 := fnv.New64a()
        h1.Write(data)
        h1v := h1.Sum64()

        h2 := fnv.New64()
        h2.Write(data)
        h2.Write([]byte{0x5A}) // different prefix for second hash
        h2v := h2.Sum64()
        if h2v == 0 {
                h2v = 1 // avoid 0
        }

        return int((h1v + uint64(seed)*h2v) % uint64(b.m))
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
