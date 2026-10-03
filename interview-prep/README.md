# Interview Prep — Senior-Level Coding Questions

A comprehensive collection of **550 hard, senior-level interview questions** across 9 categories, with solution implementations, tests, and benchmarks in **Go, Rust, and C++**.

## Categories

| # | Category | Questions | Implemented |
|---|----------|-----------|-------------|
| 1 | Concurrency & Parallelism | 60 | 10 |
| 2 | Data Structures | 70 | 10 |
| 3 | Algorithms | 90 | 10 |
| 4 | SQL & Database Design | 65 | 0 |
| 5 | System Design | 75 | 0 |
| 6 | Memory Management | 50 | 0 |
| 7 | Performance & Profiling | 50 | 0 |
| 8 | Security | 50 | 0 |
| 9 | Networking & Protocols | 40 | 0 |
| | **Total** | **550** | **30** |

## Project Structure

```
interview-prep/
├── questions.json          # All 550 questions with metadata
├── go/                     # Go solutions (30 implemented, all tested)
│   ├── concurrency/        # Lock-free stacks, queues, spinlocks, etc.
│   ├── datastructures/     # Skip lists, B-trees, caches, tries, etc.
│   └── algorithms/         # DP, graph algorithms, string matching, etc.
├── rust/                   # Rust solutions (source only — no compiler in build env)
│   └── src/
│       ├── concurrency/
│       ├── datastructures/
│       └── algorithms/
├── cpp/                    # C++ solutions (7 implemented, all tested)
│   ├── include/            # Header-only implementations
│   ├── src/                 # Source stubs
│   └── tests/              # Test file
└── scripts/                # Code generation scripts
```

## Implemented Questions (30)

### Concurrency (10)
1. **Treiber Stack** — Lock-free CAS stack (Go, Rust, C++)
2. **Michael-Scott Queue** — Lock-free MPMC queue (Go)
3. **MPSC Queue** — Lock-free multi-producer single-consumer (Go, Rust)
4. **SPSC Ring Buffer** — Bounded single-producer single-consumer (Go, Rust)
5. **Ticket Spinlock** — FIFO fair spinlock (Go)
6. **Readers-Writer Lock** — Reader-preference RW lock (Go)
7. **Condition Variable** — Wait/notify on sync.Cond (Go)
8. **Chase-Lev Work-Stealing Deque** — Owner push/pop, thief steal (Go)
9. **Counting Semaphore** — Fast-path + blocking slow path (Go, Rust)
10. **Concurrent Hash Map** — Lock-striping hash map (Go)

### Data Structures (10)
11. **Skip List** — Probabilistic balanced structure (Go, Rust)
12. **LRU Cache** — O(1) get/put via hash + linked list (Go, Rust, C++)
13. **LFU Cache** — O(1) via frequency buckets (Go)
14. **Bloom Filter** — Probabilistic set membership (Go, Rust)
15. **Disjoint Set (Union-Find)** — Path compression + union by rank (Go, Rust, C++)
16. **Segment Tree** — Range update/query with lazy propagation (Go, Rust)
17. **Fenwick Tree (BIT)** — Prefix-sum in O(log n) (Go, Rust)
18. **Trie (Patricia)** — Path-compressed trie (Go)
19. **B-Tree** — Balanced multi-way search tree (Go)
20. **Red-Black Tree** — Self-balancing BST (Go)

### Algorithms (10)
21. **Edit Distance (Levenshtein)** — O(n*m) DP, O(min) space (Go, Rust, C++)
22. **Longest Increasing Subsequence** — O(n log n) patience sort (Go, Rust, C++)
23. **Dijkstra's Algorithm** — Shortest path with indexed PQ (Go, Rust)
24. **KMP String Matching** — O(n+m) with failure function (Go, Rust)
25. **Convex Hull** — Andrew's monotone chain O(n log n) (Go)
26. **0/1 Knapsack** — Classic DP (Go, Rust)
27. **Max Flow (Dinic's)** — Level graph + blocking flow (Go)
28. **Floyd-Warshall** — All-pairs shortest paths (Go)
29. **Quickselect** — k-th smallest in expected O(n) (Go, Rust, C++)
30. **Manacher's** — Longest palindrome in O(n) (Go)

## Running

### Go
```bash
cd go
go test ./...                    # Run all tests
go test -bench=. -benchmem ./... # Run benchmarks
go test ./concurrency/ -v       # Run specific package
```

### Rust
```bash
cd rust
cargo test                      # Run all tests
cargo bench                     # Run benchmarks
```

### C++
```bash
cd cpp
make            # Build
make test       # Run tests
```

## Adding More Questions

1. Edit `questions.json` and set `"implemented": true` for new questions
2. Create the solution file in `go/`, `rust/`, and `cpp/`
3. Add tests and benchmarks
4. Run the test suite to verify

## License

MIT
