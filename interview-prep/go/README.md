# Interview Prep — Go Solutions

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
