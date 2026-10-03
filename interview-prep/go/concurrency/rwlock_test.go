package concurrency

import (
        "sync"
        "sync/atomic"
        "testing"
        "time"
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
                        // Hold the lock briefly to ensure overlap
                        time.Sleep(time.Millisecond)
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
