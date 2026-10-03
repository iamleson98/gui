package algorithms

import "testing"

func TestMaxFlowBasic(t *testing.T) {
        mf := NewMaxFlow(4)
        mf.AddEdge(0, 1, 3)
        mf.AddEdge(0, 2, 2)
        mf.AddEdge(1, 2, 1)
        mf.AddEdge(1, 3, 2)
        mf.AddEdge(2, 3, 3)
        flow := mf.MaxFlow(0, 3)
        // Max flow = min cut = 5 ({0} vs rest: 3+2=5, or {0,1} vs {2,3}: 2+1+2=5)
        if flow != 5 {
                t.Fatalf("expected 5, got %d", flow)
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
