#!/usr/bin/env python3
"""Generator for questions.json: 550 senior-level interview questions."""
import json

# Each category is a tuple of (start_id, [ (title, description, concepts, implemented_bool) ... ])
# implemented_bool is True only for the 30 representative questions specified.

CATEGORIES = []


# ---------------------------------------------------------------------------
# 1. Concurrency  (ids 1-60, 60 questions)
# ---------------------------------------------------------------------------
concurrency = [
    ("Lock-Free MPSC Queue", "Implement a single-producer, multi-consumer queue using only atomic CAS for the consumer side while producers own their nodes.", ["atomics", "CAS", "MPSC", "ABA"], True),
    ("Michael-Scott Lock-Free MPMC Queue", "Build a wait-free enqueue / lock-free dequeue linked queue using compare-and-swap on head and tail pointers.", ["lock-free", "CAS", "hazard pointers", "MPMC"], True),
    ("Lock-Free Treiber Stack", "Implement a stack whose push and pop use a single compare-and-swap on the top pointer with a tagged next field.", ["lock-free", "CAS", "ABA", "stack"], True),
    ("MCS Lock (Mellor-Crummy & Scott)", "Implement a scalable list-based queue lock where each thread spins on a locally-cached flag.", ["spinlock", "queue lock", "scalability", "NUMA"], False),
    ("CLH Lock (Craig, Landin, Hagersten)", "Build a queue lock whose thread spins on the predecessor's lock word and hands off ownership by toggling its own node.", ["spinlock", "queue lock", "FIFO", "spin locality"], False),
    ("SeqLock (Sequence Lock)", "Implement a sequence-lock reader/writer pattern allowing lock-free reads while writes increment a counter twice.", ["seqlock", "readers-writers", "memory ordering", "fences"], False),
    ("Adaptive Spinlock", "Design a spinlock that spins briefly then falls back to a kernel futex or parking primitive to avoid wasted CPU.", ["spinlock", "futex", "backoff", "hybrid"], False),
    ("Read-Copy-Update (RCU) Pattern", "Simulate RCU by allowing readers to proceed without locks and deferring reclamation to a grace period.", ["RCU", "grace period", "deferred reclamation", "read-mostly"], False),
    ("Hazard Pointers", "Implement hazard pointers so that a lock-free data structure safely defers reclamation of nodes a reader is inspecting.", ["hazard pointers", "memory reclamation", "ABA", "lock-free"], False),
    ("Epoch-Based Reclamation", "Build an epoch-based memory reclamation scheme that frees nodes only after all pre-epoch readers have retired.", ["epoch reclamation", "garbage collection", "lock-free", "ABA"], False),
    ("Ticket Spinlock", "Implement a FIFO ticket lock using fetch-add on a next ticket and a now-serving counter.", ["spinlock", "FIFO", "fetch-add", "fairness"], True),
    ("Queue Spinlock (Linux qspinlock)", "Design a compact queue spinlock that stores waiting nodes in a small per-CPU array and falls back to a linked list.", ["spinlock", "queue lock", "MCS", "fairness"], False),
    ("Peterson's Algorithm", "Implement the classic two-process mutual exclusion algorithm using flags and a turn variable with sequential consistency.", ["mutual exclusion", "flags", "turn", "memory ordering"], False),
    ("Readers-Writer Lock", "Implement a biased readers-writer lock allowing many concurrent readers or a single exclusive writer with bounded fairness.", ["readers-writers", "fairness", "biasing", "atomic counter"], True),
    ("Biasable Readers-Writer Lock", "Design an RW lock that can be biased toward readers or writers and rebiased at runtime to tune throughput.", ["readers-writers", "biasing", "throughput", "fairness"], False),
    ("Condition Variable", "Implement a condition variable over a mutex with wait/notify_one/notify_all that avoids lost wakeups and spurious returns.", ["condition variable", "mutex", "spurious wakeup", "futex"], True),
    ("Eventfd / Event Signaling", "Build an eventfd-like counter used for cross-thread signaling with overflow protection and level/edge semantics.", ["eventfd", "signaling", "counter", "edge-trigger"], False),
    ("Futex (Fast Userspace Mutex)", "Implement a userspace mutex that spins on an atomic word and parks in the kernel only on contention.", ["futex", "mutex", "kernel parking", "wait queue"], False),
    ("Mutex with Spin-then-Block", "Design a mutex that spins briefly in userspace and only then issues a system call to park the thread.", ["mutex", "spin-then-block", "futex", "latency"], False),
    ("Recursive (Reentrant) Mutex", "Implement a mutex that allows the same thread to acquire it multiple times by tracking an owner and recursion count.", ["mutex", "reentrant", "owner", "recursion count"], False),
    ("Work-Stealing Deque (Chase-Lev)", "Implement a Chase-Lev work-stealing deque with a bounded circular buffer, steal-half, and double-buffering of indices.", ["work stealing", "deque", "Chase-Lev", "cas"], True),
    ("Lock-Free SPSC Ring Buffer", "Build a single-producer single-consumer bounded ring buffer using relaxed loads/stores and a power-of-two size.", ["SPSC", "ring buffer", "memory ordering", "cache lines"], False),
    ("Bounded MPMC Queue (Array)", "Implement an array-based bounded MPMC queue using a sequence per cell and compare-and-swap on the cell.", ["MPMC", "bounded queue", "sequence", "CAS"], False),
    ("Unbounded Lock-Free Queue with SMR", "Design an unbounded MPMC queue that grows linked-node storage and reclaims nodes via hazard pointers or epochs.", ["lock-free queue", "hazard pointers", "ABA", "unbounded"], False),
    ("ABA Problem and Tagged Pointers", "Demonstrate the ABA problem on a Treiber stack and fix it using a tagged pointer packing a version counter.", ["ABA", "tagged pointer", "CAS", "versioning"], False),
    ("DCAS / Double-Width CAS", "Implement a 128-bit compare-and-swap (DCAS) to atomically update a pointer and a counter together.", ["DCAS", "double-width CAS", "versioning", "portability"], False),
    ("Memory Ordering: Acquire/Release vs seq_cst", "Compare acquire/release, relaxed, and sequentially-consistent ordering and pick the weakest safe ordering per access.", ["memory ordering", "acquire/release", "seq_cst", "reordered"], False),
    ("Memory Barriers and Fences", "Place read and write fences correctly so that lock-free algorithms publish visibility and consumption in the intended order.", ["memory fence", "load-store", "visibility", "portability"], False),
    ("Wait-Free vs Lock-Free Progress", "Design a wait-free queue where every operation completes in a bounded number of steps, and contrast with lock-free progress.", ["wait-free", "lock-free", "progress", "bounded steps"], False),
    ("Semaphore", "Implement a counting semaphore over a futex/condvar with wait and post operations and a bounded count.", ["semaphore", "counting", "park", "futex"], True),
    ("Counting Semaphore with Futex", "Build a fast counting semaphore whose fast path is an atomic compare and whose slow path parks waiters.", ["semaphore", "futex", "fast path", "wait queue"], False),
    ("SPSC Ring Buffer", "Implement a bounded single-producer single-consumer ring buffer with relaxed atomics and cache-line padding.", ["SPSC", "ring buffer", "relaxed atomics", "false sharing"], True),
    ("LMAX Disruptor Pattern", "Build a Disruptor-style ring with sequenced consumers, gating sequences, and a batched publisher.", ["Disruptor", "ring", "sequences", "batching"], False),
    ("Concurrent Hash Map with Lock Striping", "Implement a hash map sharded by a fixed array of striped locks, resizing with global rehash.", ["hash map", "lock striping", "resizing", "sharding"], True),
    ("Concurrent Hash Map with CAS Buckets", "Build a hash map whose buckets are lock-free singly linked lists updated by compare-and-swap.", ["hash map", "lock-free", "CAS", "chaining"], False),
    ("Split-Ordered List", "Implement a lock-free hash table based on a sorted linked list with reverse-key ordering (Shalev-Shavit).", ["split-ordered list", "lock-free", "sorted list", "hash"], False),
    ("RCU-Protected Linked List", "Implement a linked list whose readers traverse lock-free while updaters use RCU to defer node removal.", ["RCU", "linked list", "grace period", "read-mostly"], False),
    ("Lock-Free Doubly Linked List", "Design a lock-free doubly linked list handling the classic concurrent-deletion hazard with marking.", ["doubly linked list", "lock-free", "marking", "ABA"], False),
    ("Concurrent Skip List", "Implement a lock-free or fine-grained skip list supporting ordered map operations with probabilistic levels.", ["skip list", "concurrent", "probabilistic", "ordered map"], False),
    ("Wait-Free Hash Table", "Design a resize-friendly wait-free hash table where every operation completes in bounded CAS steps.", ["wait-free", "hash table", "resize", "bounded steps"], False),
    ("Async/Await Executor", "Build a single-threaded cooperative task executor with a ready queue, polling, and wakers for async/await.", ["async/await", "executor", "waker", "poll"], False),
    ("Thread Pool with Work Queue", "Implement a fixed-size worker pool with a global task queue, blocking dequeue, and shutdown semantics.", ["thread pool", "work queue", "workers", "shutdown"], False),
    ("Fork-Join Pool", "Design a fork-join executor with work-stealing deques, barrier joins, and recursive task splitting.", ["fork-join", "work stealing", "recursive", "divide and conquer"], False),
    ("Actor Model Mailbox", "Implement an actor runtime with per-actor mailboxes, message ordering, and a dispatcher.", ["actor model", "mailbox", "message passing", "dispatcher"], False),
    ("CSP Channels (Go-style)", "Build unbuffered and buffered channels with select, close, and fair rendezvous semantics.", ["channels", "CSP", "select", "rendezvous"], False),
    ("Producer-Consumer Bounded Buffer", "Implement the classic producer-consumer bounded buffer using a mutex and two condition variables.", ["producer-consumer", "bounded buffer", "condition variable", "backpressure"], False),
    ("Dining Philosophers", "Solve the dining philosophers using resource hierarchy, a waiter (arbitrator), and Chandy-Misra messages.", ["deadlock", "resource hierarchy", "arbitrator", "fairness"], False),
    ("Readers-Writers with Writer Preference", "Design an RW lock that prefers writers to avoid writer starvation while preventing reader starvation.", ["readers-writers", "writer preference", "starvation", "fairness"], False),
    ("Barrier Synchronization", "Implement a reusable barrier where N threads wait and then all proceed, with sense reversal.", ["barrier", "sense reversal", "reuse", "wait"], False),
    ("Phaser / Cyclic Barrier", "Design a phaser supporting dynamic party registration, arrivals, and phase advancement.", ["phaser", "cyclic barrier", "parties", "phases"], False),
    ("CountDownLatch Equivalent", "Implement a one-shot latch that blocks threads until a counter reaches zero.", ["latch", "one-shot", "counter", "park"], False),
    ("Exchanger", "Build an exchanger where two threads rendezvous and swap values atomically.", ["exchanger", "rendezvous", "swap", "two threads"], False),
    ("Futures and Promises", "Implement a future/promise pair with shared state, continuations, and ready/error/pending states.", ["future", "promise", "continuation", "shared state"], False),
    ("Once / Call-Once Initialization", "Implement std::once / sync.Once semantics guaranteeing a function runs exactly once under concurrency.", ["once", "initialization", "double-checked", "atomic"], False),
    ("Double-Checked Locking", "Implement a correct double-checked locking singleton using acquire/release fences to avoid the classic pitfall.", ["double-checked locking", "singleton", "memory ordering", "fence"], False),
    ("Thread-Local Storage", "Design a thread-local storage abstraction with per-thread slots and optional cleanup on thread exit.", ["thread-local", "slots", "cleanup", "per-thread"], False),
    ("Async Mutex / Async-Aware Lock", "Implement a mutex whose waiters park futures rather than OS threads, avoiding thread blocking.", ["async", "mutex", "wait list", "parking"], False),
    ("Exponential Backoff Strategies", "Build an exponential backoff with jitter for retrying contended CAS loops and RPC calls.", ["backoff", "exponential", "jitter", "contention"], False),
    ("NUMA-Aware Synchronization", "Design locks and data placement that respect NUMA topology to reduce cross-socket traffic.", ["NUMA", "topology", "data placement", "scalability"], False),
    ("Coherence Traffic and False Sharing", "Diagnose false sharing between adjacent atomics on the same cache line and pad to eliminate coherence traffic.", ["false sharing", "cache line", "padding", "coherence"], False),
]

CATEGORIES.append(("Concurrency", 1, concurrency))


# ---------------------------------------------------------------------------
# 2. Data Structures  (ids 61-130, 70 questions)
# ---------------------------------------------------------------------------
data_structures = [
    ("AVL Tree", "Implement a self-balancing binary search tree that maintains height balance via rotations on insert and delete.", ["BST", "rotations", "balance factor", "AVL"], False),
    ("Splay Tree", "Build a self-adjusting BST that moves recently accessed nodes to the root via zig, zig-zig, and zig-zag operations.", ["splay tree", "self-adjusting", "amortized", "rotations"], False),
    ("AA Tree", "Implement a balanced BST using horizontal/vertical links and skew/split rebalancing for simpler code than red-black.", ["AA tree", "level", "skew", "split"], False),
    ("B+ Tree", "Build a disk-oriented B+ tree with leaf links, internal node fan-out, and range scan support.", ["B+ tree", "fan-out", "leaf links", "range scan"], False),
    ("B* Tree", "Implement a B-tree variant that keeps nodes at least two-thirds full by redistributing before splitting.", ["B* tree", "redistribution", "split", "fill factor"], False),
    ("2-3-4 Tree", "Build a balanced search tree with 2-, 3-, and 4-nodes that splits on overflow from the bottom up.", ["2-3-4 tree", "splitting", "bottom-up", "balance"], False),
    ("Scapegoat Tree", "Implement a self-balancing BST that rebuilds an unbalanced subtree when its height exceeds a logarithmic bound.", ["scapegoat tree", "rebuild", "amortized", "alpha-balanced"], False),
    ("Treap (Tree + Heap)", "Build a randomized BST that maintains heap order on randomly assigned priorities.", ["treap", "randomized", "priority", "rotations"], False),
    ("Cartesian Tree", "Construct a Cartesian tree from an array in linear time and use it for range minimum queries.", ["Cartesian tree", "RMQ", "heap property", "linear build"], False),
    ("Ternary Search Trie", "Implement a ternary search trie for string keys with character-by-character branching.", ["ternary trie", "strings", "prefix", "branching"], False),
    ("Suffix Tree (Ukkonen)", "Build Ukkonen's linear-time suffix tree with implicit suffix links and active point extension.", ["suffix tree", "Ukkonen", "implicit links", "linear time"], False),
    ("Suffix Array", "Construct a suffix array in O(n log n) and demonstrate binary search over it.", ["suffix array", "SA-IS", "binary search", "strings"], False),
    ("Suffix Automaton", "Build the minimal DFA of all suffixes of a string for online substring queries.", ["suffix automaton", "DFA", "endpos", "online"], False),
    ("Rope (String)", "Implement a rope as a balanced binary tree of string chunks supporting split, concat, and insert.", ["rope", "balanced tree", "split", "chunks"], False),
    ("Van Emde Boas Tree", "Build a vEB tree over a fixed universe for O(log log U) insert, successor, and predecessor.", ["vEB tree", "log log U", "cluster", "universe"], False),
    ("Interval Tree", "Implement a red-black based interval tree supporting overlap queries for intervals.", ["interval tree", "overlap query", "augmented BST", "red-black"], False),
    ("Binary Indexed Range Tree", "Build a BIT supporting range updates and point queries via two complementary arrays.", ["BIT", "range update", "point query", "difference"], False),
    ("Binary Heap", "Implement a binary heap as an array with sift-up and sift-down for priority queue operations.", ["binary heap", "array", "sift", "priority queue"], False),
    ("Binomial Heap", "Build a binomial heap supporting merge in O(log n) using linked binomial trees.", ["binomial heap", "merge", "lazy", "amortized"], False),
    ("Fibonacci Heap", "Implement a Fibonacci heap with lazy melding and amortized O(1) decrease-key for Dijkstra.", ["Fibonacci heap", "amortized", "decrease-key", "cascading cut"], False),
    ("Pairing Heap", "Build a pairing heap that achieves practical speed via two-pass merging of children.", ["pairing heap", "merge", "two-pass", "amortized"], False),
    ("B-Tree", "Implement a B-tree with configurable minimum degree, node splitting, and merging for disk-backed storage.", ["B-tree", "minimum degree", "split", "merge"], True),
    ("K-D Tree", "Build a k-d tree for orthogonal range and nearest-neighbor queries in k-dimensional space.", ["k-d tree", "nearest neighbor", "range query", "splitting planes"], False),
    ("Skip List", "Implement a probabilistic skip list with random levels supporting O(log n) search and range queries.", ["skip list", "probabilistic", "levels", "ordered map"], True),
    ("Compressed Trie / Patricia", "Build a Patricia (radix) trie that compresses single-child chains for compact string storage.", ["Patricia trie", "compression", "strings", "radix"], True),
    ("DAWG (Directed Acyclic Word Graph)", "Build a minimal acyclic DFA accepting all suffixes of a string via suffix-tree compression.", ["DAWG", "minimal DFA", "suffix links", "strings"], False),
    ("Aho-Corasick Automaton", "Construct the AC automaton with failure links for multi-pattern string matching.", ["Aho-Corasick", "failure links", "multi-pattern", "DFA"], False),
    ("Wavelet Tree", "Implement a wavelet tree for rank/select queries over a sequence using bit vectors.", ["wavelet tree", "rank/select", "bit vector", "sequence"], False),
    ("Segment Tree with Lazy Propagation", "Build a segment tree supporting range updates and range queries with lazy propagation.", ["segment tree", "lazy propagation", "range update", "range query"], True),
    ("Fenwick Tree / BIT", "Implement a binary indexed tree for prefix sums and point updates in O(log n).", ["Fenwick tree", "BIT", "prefix sum", "lowbit"], True),
    ("Sparse Table (RMQ)", "Preprocess an array for O(1) range minimum queries using a sparse table of powers of two.", ["sparse table", "RMQ", "idempotent", "preprocessing"], False),
    ("Disjoint Sparse Table", "Build a sparse table supporting non-idempotent range queries such as sum in O(log n).", ["sparse table", "non-idempotent", "range sum", "preprocessing"], False),
    ("Cartesian Tree for RMQ", "Reduce RMQ to LCA on a Cartesian tree built from the array in linear time.", ["Cartesian tree", "LCA", "RMQ reduction", "Euler tour"], False),
    ("Link-Cut Tree (Splay)", "Implement a splay-based link-cut tree supporting dynamic forest queries and edge link/cut.", ["link-cut tree", "splay", "dynamic forest", "preferred path"], False),
    ("Euler Tour Tree", "Build an Euler tour tree over a balanced BST to support dynamic connectivity.", ["Euler tour tree", "balanced BST", "dynamic connectivity", "edge"], False),
    ("Red-Black Tree", "Implement a red-black BST with color flips and rotations to keep O(log n) height on insert/delete.", ["red-black tree", "color flip", "rotations", "invariants"], True),
    ("Bloom Filter", "Implement a Bloom filter with k hash functions and analyze false-positive probability.", ["Bloom filter", "false positive", "hash functions", "bitset"], True),
    ("Counting Bloom Filter", "Extend a Bloom filter to counters so it supports deletion via reference counting.", ["counting Bloom filter", "counters", "deletion", "false positive"], False),
    ("Cuckoo Filter", "Build a cuckoo-filter using bounded cuckoo hashing with fingerprints for set membership and deletion.", ["cuckoo filter", "fingerprint", "cuckoo hashing", "deletion"], False),
    ("HyperLogLog", "Implement HyperLogLog for approximate distinct-count (cardinality) estimation in fixed memory.", ["HyperLogLog", "cardinality", "stochastic averaging", "bias correction"], False),
    ("LRU Cache", "Implement an O(1) LRU cache using a hash map plus a doubly linked list of access order.", ["LRU cache", "doubly linked list", "hash map", "eviction"], True),
    ("LFU Cache", "Build an O(1) LFU cache tracking frequency buckets and reordering on access.", ["LFU cache", "frequency bucket", "eviction", "hash map"], True),
    ("ARC (Adaptive Replacement Cache)", "Implement ARC, which dynamically balances recency and frequency between LRU and LFU.", ["ARC", "adaptive", "recency", "frequency"], False),
    ("Disjoint Set Union-Find", "Implement union-find with path compression and union by rank for near-constant amortized operations.", ["union-find", "path compression", "union by rank", "DSU"], True),
    ("Union-Find with Rollback", "Extend DSU to support undo of union operations for backtracking and offline algorithms.", ["union-find", "rollback", "persistent", "offline"], False),
    ("Link-Cut Dynamic Connectivity", "Use link-cut trees to maintain connected components under edge insertions and deletions.", ["dynamic connectivity", "link-cut", "fully dynamic", "forest"], False),
    ("Persistent Segment Tree", "Build a segment tree that versions on update by sharing unchanged nodes.", ["persistent segment tree", "versioning", "node sharing", "immutability"], False),
    ("Persistent Treap", "Implement an implicit-key treap that persists prior versions on split and merge.", ["persistent treap", "split/merge", "immutability", "randomized"], False),
    ("Persistent Array", "Build a persistent array using a fat-node or balanced-tree representation with O(log n) updates.", ["persistent array", "fat node", "versioning", "immutability"], False),
    ("Fully Persistent BST", "Implement a BST that keeps every prior version queryable after updates.", ["persistent BST", "path copying", "versioning", "immutability"], False),
    ("Chunking Deque", "Build a deque over fixed-size chunks (a 'banker's deque') for amortized O(1) operations.", ["deque", "chunking", "amortized", "persistent"], False),
    ("Finger Tree", "Implement Okasaki's finger tree supporting amortized O(1) cons/snoc and O(log n) split.", ["finger tree", "amortized", "split", "persistent"], False),
    ("VList", "Implement the VList structure providing O(1) cons and O(log n) indexing using linked blocks.", ["VList", "linked blocks", "persistent", "indexing"], False),
    ("Unrolled Linked List", "Build a linked list whose nodes store multiple elements to improve cache locality.", ["unrolled list", "cache locality", "node capacity", "linked list"], False),
    ("XOR Linked List", "Implement a doubly linked list using XOR of adjacent pointers to store one pointer per node.", ["XOR list", "pointer compression", "memory", "traversal"], False),
    ("Skip List with Finger Search", "Extend a skip list with finger search to find elements near a recent position faster.", ["skip list", "finger", "local search", "ordered map"], False),
    ("Binary Search Tree (Basic)", "Implement an unbalanced BST with insert, delete, and search.", ["BST", "inorder", "insert/delete", "balanced"], False),
    ("Implicit Treap (Split/Merge)", "Build a treap keyed by subtree size supporting split and merge for sequence operations.", ["implicit treap", "split", "merge", "subtree size"], False),
    ("Randomized BST", "Implement a randomized BST that inserts at the root with probability 1/n to stay balanced in expectation.", ["randomized BST", "expected balance", "root insert", "probability"], False),
    ("Count-Min Sketch", "Implement the Count-Min sketch for approximate frequency estimation with d hash functions.", ["Count-Min sketch", "frequency", "hash functions", "overestimate"], False),
    ("Quotient Filter", "Build a quotient filter using quotient/remainder hashing for membership with locality.", ["quotient filter", "open addressing", "locality", "membership"], False),
    ("R-Tree (Spatial)", "Implement an R-tree for spatial indexing of rectangles with node splitting heuristics.", ["R-tree", "spatial index", "MBR", "splitting"], False),
    ("Hilbert R-Tree", "Design an R-tree whose leaves follow a Hilbert ordering to improve query performance.", ["Hilbert R-tree", "space-filling curve", "ordering", "spatial"], False),
    ("LSM-Tree (Log-Structured Merge)", "Implement a log-structured merge tree with memtable, SSTables, and leveled compaction.", ["LSM tree", "memtable", "SSTable", "compaction"], False),
    ("Fractal Tree (TokuDB)", "Build a fractal index tree using buffered insertions to amortize I/O across internal nodes.", ["fractal tree", "buffered", "amortized I/O", "B-tree"], False),
    ("Hash Array Mapped Trie (HAMT)", "Implement a HAMT using a sparse bitmap and a variable-length pointer array per node.", ["HAMT", "bitmap", "hash trie", "persistent"], False),
    ("RRB-Trees (Relaxed Radix Balanced)", "Build relaxed radix-balanced trees for efficient concat and split on immutable vectors.", ["RRB tree", "relaxed", "concat", "immutable vector"], False),
    ("Clojure-style Persistent Vector", "Implement a persistent vector using a bitmapped trie indexed by chunks.", ["persistent vector", "bitmapped trie", "tail", "immutability"], False),
    ("Bitmapped Vector Trie", "Build a trie with bitmap per node and branching factor of word size for persistent arrays.", ["vector trie", "bitmap", "branching", "persistent"], False),
    ("Merkle Tree", "Implement a Merkle tree of content hashes supporting inclusion proofs and tamper detection.", ["Merkle tree", "hash", "inclusion proof", "tamper detection"], False),
]

CATEGORIES.append(("Data Structures", 61, data_structures))


# ---------------------------------------------------------------------------
# 3. Algorithms  (ids 131-220, 90 questions)
# ---------------------------------------------------------------------------
algorithms = [
    ("Quicksort with 3-Way Partitioning", "Implement quicksort using Dutch national flag partitioning to handle duplicates efficiently.", ["quicksort", "3-way", "duplicates", "in-place"], False),
    ("Introsort", "Build a hybrid sort that switches from quicksort to heapsort on recursion depth to guarantee O(n log n).", ["introsort", "hybrid", "heapsort", "worst-case"], False),
    ("TimSort", "Implement TimSort with run detection, merging, and galloping for partially ordered real-world data.", ["TimSort", "runs", "galloping", "adaptive"], False),
    ("External Merge Sort", "Sort datasets larger than memory using k-way merging of sorted runs on disk.", ["external sort", "k-way merge", "runs", "I/O"], False),
    ("Heap Sort", "Implement in-place heapsort with a build-heap linear phase and repeated extract-max.", ["heap sort", "in-place", "build heap", "extract max"], False),
    ("In-Place MSD Radix Sort", "Sort strings in-place using most-significant-digit radix recursion with a key-indexed count.", ["MSD radix", "in-place", "recursion", "strings"], False),
    ("LSD Radix Sort", "Implement least-significant-digit radix sort using counting sort per digit.", ["LSD radix", "counting sort", "stable", "fixed width"], False),
    ("Counting Sort (Stable)", "Build a stable counting sort over a small integer key domain in O(n + k).", ["counting sort", "stable", "O(n+k)", "integers"], False),
    ("American Flag Sort", "Implement an in-place MSD radix variant using partition pointers per bucket.", ["American flag sort", "in-place", "MSD", "radix"], False),
    ("Shell Sort with Ciura Gaps", "Implement shellsort using Ciura's empirically tuned gap sequence.", ["shell sort", "gaps", "Ciura", "in-place"], False),
    ("Median of Medians (BFPRT)", "Implement linear-time selection using the median-of-medians pivot strategy with guaranteed bounds.", ["BFPRT", "selection", "median of medians", "linear"], False),
    ("Binary Search (Lower/Upper Bound)", "Implement lower_bound and upper_bound over sorted arrays with half-open intervals.", ["binary search", "lower bound", "upper bound", "sorted"], False),
    ("Exponential Search", "Search sorted arrays by doubling the index then binary searching within the bounded range.", ["exponential search", "doubling", "unbounded", "sorted"], False),
    ("Interpolation Search", "Implement interpolation search for uniformly distributed keys, achieving O(log log n) on average.", ["interpolation search", "uniform", "probe", "sorted"], False),
    ("Ternary Search (Unimodal)", "Find the maximum of a unimodal function by repeatedly narrowing with two probes.", ["ternary search", "unimodal", "divide", "optimization"], False),
    ("Fractional Cascading", "Speed up multi-level binary searches by cascading a fraction of elements between levels.", ["fractional cascading", "multi-level", "binary search", "amortized"], False),
    ("Fractional Knapsack (Greedy)", "Solve the fractional knapsack by sorting items by value/weight and greedily filling.", ["fractional knapsack", "greedy", "value/weight", "sort"], False),
    ("Activity Selection", "Solve interval scheduling by greedily picking the earliest-finishing compatible activity.", ["greedy", "intervals", "earliest finish", "optimal"], False),
    ("Huffman Coding", "Build an optimal prefix code using a min-heap and repeated merges of the two least-frequent symbols.", ["Huffman", "prefix code", "greedy", "min-heap"], False),
    ("Optimal Merge Pattern", "Minimize the cost of merging sorted runs by always merging the two smallest.", ["greedy", "merge cost", "min-heap", "optimal"], False),
    ("Greedy Job Scheduling with Deadlines", "Maximize profit by scheduling unit-length jobs before their deadlines using disjoint-set slotting.", ["greedy", "deadlines", "disjoint set", "profit"], False),
    ("Kruskal's MST", "Build a minimum spanning forest using union-find to add edges in sorted order without forming cycles.", ["MST", "Kruskal", "union-find", "greedy"], False),
    ("Prim's MST", "Grow an MST from a start vertex using a priority queue of crossing edges.", ["MST", "Prim", "priority queue", "greedy"], False),
    ("Boruvka's MST", "Compute MST by iteratively adding the cheapest edge from every component.", ["MST", "Boruvka", "components", "parallel"], False),
    ("Reverse Delete MST", "Build MST by deleting the heaviest edge that does not disconnect the graph.", ["MST", "reverse delete", "cycle", "greedy"], False),
    ("Topological Sort (Kahn)", "Produce a topological ordering of a DAG using in-degree counts and a queue.", ["topological sort", "Kahn", "in-degree", "DAG"], False),
    ("Topological Sort (DFS)", "Generate a topological order by post-order DFS and reversing the finish times.", ["topological sort", "DFS", "post-order", "DAG"], False),
    ("Tarjan's SCC", "Find strongly connected components in linear time using a DFS stack and lowlink values.", ["SCC", "Tarjan", "lowlink", "DFS"], False),
    ("Kosaraju's SCC", "Compute SCCs by running DFS on the graph and then on the reverse graph in decreasing finish order.", ["SCC", "Kosaraju", "reverse graph", "finish order"], False),
    ("Gabow's SCC", "Implement Gabow's path-based SCC algorithm using two stacks and a path index counter.", ["SCC", "Gabow", "path-based", "linear"], False),
    ("2-SAT (Implication Graph)", "Solve 2-SAT by reducing to SCC detection on the implication graph and checking variable order.", ["2-SAT", "implication graph", "SCC", "negation"], False),
    ("Edit Distance", "Compute Levenshtein edit distance with a dynamic programming table and optional space reduction to two rows.", ["edit distance", "Levenshtein", "dynamic programming", "space reduction"], True),
    ("LIS in O(n log n)", "Compute the longest increasing subsequence using a patience-sorting tail array and binary search.", ["LIS", "patience sorting", "binary search", "O(n log n)"], True),
    ("Longest Common Subsequence", "Build the LCS dynamic programming table and reconstruct the subsequence via backtracking.", ["LCS", "dynamic programming", "backtracking", "suffix"], False),
    ("Knapsack 0/1", "Solve the 0/1 knapsack with dynamic programming and reduce space to a single row.", ["knapsack", "0/1", "dynamic programming", "space reduction"], True),
    ("Unbounded Knapsack", "Solve the unbounded knapsack where items can be reused with a 1D DP.", ["knapsack", "unbounded", "1D DP", "reuse"], False),
    ("Matrix Chain Multiplication", "Find the parenthesization minimizing scalar multiplications using interval DP.", ["matrix chain", "interval DP", "parenthesization", "cost"], False),
    ("Manacher's Longest Palindrome", "Find the longest palindromic substring in linear time using a centered radius array.", ["palindrome", "Manacher", "linear", "radius"], True),
    ("Palindrome Partitioning (DP)", "Minimize cuts needed to partition a string into palindromes using precomputed palindrome tables.", ["palindrome", "partition", "dynamic programming", "cuts"], False),
    ("Subset Sum (Pseudo-Polynomial)", "Solve subset sum using a bitset DP over the achievable sums.", ["subset sum", "bitset", "pseudo-polynomial", "DP"], False),
    ("Partition Equal Subset Sum", "Determine if an array can be partitioned into two equal-sum subsets using subset-sum DP.", ["partition", "subset sum", "DP", "boolean"], False),
    ("Egg Drop (DP)", "Find the minimum number of egg-drop trials in the worst case using a DP over eggs and floors.", ["egg drop", "DP", "worst case", "trials"], False),
    ("Minimum Insertions for Palindrome", "Compute the minimum insertions to make a string a palindrome using LCS with its reverse.", ["palindrome", "insertions", "LCS", "DP"], False),
    ("Word Break (DP)", "Determine whether a string can be segmented into dictionary words using DP.", ["word break", "DP", "trie", "segmentation"], False),
    ("Hamiltonian Path (TSP Bitmask DP)", "Solve the traveling salesperson problem with a Held-Karp bitmask DP over subsets.", ["TSP", "Held-Karp", "bitmask", "DP"], False),
    ("Bellman-Ford", "Compute shortest paths with negative weights using edge relaxation and a negative-cycle detector.", ["shortest path", "Bellman-Ford", "negative weights", "relaxation"], False),
    ("Dijkstra with Decrease-Key", "Implement Dijkstra using a binary heap with a decrease-key operation for efficient relaxation.", ["shortest path", "Dijkstra", "binary heap", "decrease-key"], True),
    ("A* Search", "Implement A* with a consistent heuristic to find shortest paths faster than Dijkstra.", ["A*", "heuristic", "priority queue", "shortest path"], False),
    ("Johnson's All-Pairs", "Compute all-pairs shortest paths by reweighting with Bellman-Ford then running Dijkstra per vertex.", ["all-pairs", "Johnson", "reweighting", "Dijkstra"], False),
    ("SPFA (Shortest Path Faster)", "Implement the queue-based Bellman-Ford variant that only relaxes vertices whose distance changed.", ["SPFA", "queue", "relaxation", "negative weights"], False),
    ("Floyd-Warshall APSP", "Compute all-pairs shortest paths with a three-loop DP over intermediate vertices.", ["all-pairs", "Floyd-Warshall", "DP", "intermediate"], True),
    ("Transitive Closure (Roy-Warshall)", "Compute the transitive closure of a graph using a Floyd-Warshall-style boolean DP.", ["transitive closure", "boolean", "DP", "reachability"], False),
    ("Max Flow: Dinic's", "Implement Dinic's max flow using BFS level graphs and blocking DFS flows.", ["max flow", "Dinic", "level graph", "blocking flow"], True),
    ("Max Flow: Ford-Fulkerson", "Compute max flow by augmenting along any augmenting path until none remain.", ["max flow", "Ford-Fulkerson", "augmenting path", "residual"], False),
    ("Max Flow: Edmonds-Karp", "Implement the BFS-based shortest-augmenting-path max flow with polynomial time.", ["max flow", "Edmonds-Karp", "BFS", "shortest augmenting path"], False),
    ("Max Flow: Push-Relabel", "Compute max flow using the Goldberg-Tarjan push-relabel algorithm with a height function.", ["push-relabel", "height function", "preflow", "max flow"], False),
    ("Min-Cost Max-Flow", "Find the maximum flow of minimum cost using successive shortest paths with potentials.", ["min-cost flow", "potentials", "SPFA", "residual"], False),
    ("Bipartite Matching (Hungarian)", "Solve the assignment problem with the O(n^3) Hungarian/Kuhn-Munkres algorithm.", ["assignment", "Hungarian", "dual", "bipartite"], False),
    ("KMP String Matching", "Implement the Knuth-Morris-Pratt matcher with a failure/prefix function for linear-time search.", ["string matching", "KMP", "failure function", "linear"], True),
    ("Boyer-Moore String Matching", "Implement Boyer-Moore using bad-character and good-suffix heuristics to skip alignments.", ["string matching", "bad character", "good suffix", "skip"], False),
    ("Rabin-Karp", "Implement the Rabin-Karp rolling-hash matcher for single and multi-pattern search.", ["rolling hash", "Rabin-Karp", "collision", "multi-pattern"], False),
    ("Z-Algorithm", "Compute the Z-array of a string for pattern matching and pattern analysis in linear time.", ["Z-array", "string matching", "prefix", "linear"], False),
    ("Aho-Corasick Multi-Pattern Matching", "Build the AC automaton to find all occurrences of multiple patterns simultaneously.", ["Aho-Corasick", "failure links", "output links", "DFA"], False),
    ("Suffix Array Construction (SA-IS)", "Construct a suffix array in linear time using the SA-IS induced-sorting algorithm.", ["suffix array", "SA-IS", "induced sorting", "linear"], False),
    ("Convex Hull", "Compute the convex hull of a 2D point set using Andrew's monotone chain in O(n log n).", ["convex hull", "monotone chain", "cross product", "sort"], True),
    ("Graham Scan", "Build the convex hull by angularly sorting points and using a stack with backtracking.", ["convex hull", "Graham scan", "angular sort", "stack"], False),
    ("Andrew's Monotone Chain", "Compute the upper and lower hulls by sorting points and scanning with cross-product tests.", ["convex hull", "monotone chain", "cross product", "sort"], False),
    ("Jarvis March (Gift Wrapping)", "Build the convex hull by gift wrapping around the point set in O(nh).", ["convex hull", "gift wrapping", "orientation", "output-sensitive"], False),
    ("QuickHull", "Implement the divide-and-conquer QuickHull algorithm for the convex hull.", ["convex hull", "QuickHull", "divide and conquer", "farthest point"], False),
    ("Rotating Calipers", "Use rotating calipers on a convex polygon to compute diameter, width, and antipodal pairs.", ["rotating calipers", "antipodal", "diameter", "convex polygon"], False),
    ("Bentley-Ottmann Line Sweep", "Find all intersections of line segments using a sweep line and balanced tree of active segments.", ["line sweep", "Bentley-Ottmann", "events", "balanced tree"], False),
    ("Closest Pair of Points", "Find the closest pair of points in O(n log n) using divide and conquer across a sorted strip.", ["closest pair", "divide and conquer", "strip", "sort"], False),
    ("Fortune's Voronoi Diagram", "Construct a Voronoi diagram using Fortune's sweep-line and beach-line data structure.", ["Voronoi", "Fortune", "sweep line", "beach line"], False),
    ("Delaunay Triangulation", "Build the Delaunay triangulation maximizing the minimum angle using incremental or divide-and-conquer methods.", ["Delaunay", "triangulation", "in-circle test", "max-min angle"], False),
    ("Point in Polygon (Ray Casting)", "Test whether a point lies inside a polygon using the ray crossing number algorithm.", ["point in polygon", "ray casting", "winding number", "parity"], False),
    ("Segment Intersection Test", "Implement orientation tests to detect whether two line segments intersect, including collinear cases.", ["segment intersection", "orientation", "collinear", "geometry"], False),
    ("Ternary Search on Reals", "Find the extremum of a unimodal real-valued function using golden-section ternary search.", ["ternary search", "unimodal", "golden section", "optimization"], False),
    ("Newton-Raphson Root Finding", "Implement Newton's method with safeguards for finding roots of smooth functions.", ["Newton-Raphson", "root finding", "Jacobian", "convergence"], False),
    ("Karatsuba Multiplication", "Multiply large integers in O(n^1.585) using a divide-and-conquer three-product scheme.", ["Karatsuba", "big integer", "divide and conquer", "multiplication"], False),
    ("Fast Fourier Transform", "Implement the FFT to evaluate polynomials in O(n log n) and multiply polynomials via pointwise products.", ["FFT", "polynomial", "Cooley-Tukey", "roots of unity"], False),
    ("Number Theoretic Transform", "Implement the NTT over a prime modulus to perform exact polynomial multiplication without floating point.", ["NTT", "modular", "primitive root", "polynomial"], False),
    ("Segmented Sieve of Eratosthenes", "Generate primes in a large interval using a segmented sieve with small primes as wheels.", ["sieve", "segmented", "primes", "wheel"], False),
    ("Miller-Rabin Primality", "Implement the randomized Miller-Rabin primality test with strong pseudoprime witnesses.", ["primality", "Miller-Rabin", "witnesses", "randomized"], False),
    ("Pollard's Rho Factorization", "Factor composite integers using Pollard's rho with cycle detection and a fallback trial division.", ["factorization", "Pollard rho", "cycle detection", "randomized"], False),
    ("Quickselect", "Find the k-th smallest element in expected linear time using partitioning like quicksort.", ["selection", "partition", "expected linear", "Lomuto/Hoare"], True),
    ("Median of Two Sorted Arrays", "Find the median of two sorted arrays in O(log(min(m, n))) using binary partition.", ["median", "two arrays", "binary partition", "logarithmic"], False),
    ("K-th Smallest in a Matrix", "Find the k-th smallest element in a sorted matrix using a min-heap or binary search on value.", ["k-th smallest", "matrix", "binary search", "min-heap"], False),
    ("Count Inversions via Merge Sort", "Count array inversions in O(n log n) by augmenting merge sort with a counter.", ["inversions", "merge sort", "count", "O(n log n)"], False),
    ("Count of Range Sum", "Count subarray sums in a range using a Fenwick tree over prefix sums.", ["range sum", "prefix sum", "Fenwick tree", "count"], False),
    ("Mo's Algorithm (Offline Queries)", "Answer range queries by reordering them into sqrt-blocks for O((n+q) sqrt n) time.", ["Mo's algorithm", "offline", "sqrt decomposition", "reorder"], False),
]

CATEGORIES.append(("Algorithms", 131, algorithms))


# ---------------------------------------------------------------------------
# 4. SQL & Database Design  (ids 221-285, 65 questions)
# ---------------------------------------------------------------------------
sql = [
    ("Normalization: 1NF/2NF/3NF", "Apply first, second, and third normal forms to eliminate anomalies and redundancy in a schema.", ["normalization", "1NF", "2NF", "3NF", "anomalies"], False),
    ("Boyce-Codd Normal Form", "Identify and decompose a schema to BCNF by removing non-trivial dependencies where a determinant is not a candidate key.", ["BCNF", "functional dependency", "candidate key", "decomposition"], False),
    ("4NF and 5NF (MVD/JD)", "Handle multi-valued and join dependencies to reach 4NF and 5NF in complex schemas.", ["4NF", "5NF", "multi-valued dependency", "join dependency"], False),
    ("Denormalization Tradeoffs", "Evaluate when denormalizing for read performance outweighs the cost of update anomalies.", ["denormalization", "read performance", "anomalies", "tradeoff"], False),
    ("Entity-Relationship Modeling", "Translate an ER diagram into a normalized relational schema with keys and cardinalities.", ["ER modeling", "entities", "relationships", "cardinality"], False),
    ("Star Schema (OLAP)", "Design a star schema with a central fact table surrounded by dimension tables for OLAP queries.", ["star schema", "fact table", "dimensions", "OLAP"], False),
    ("Snowflake Schema", "Normalize dimensions in a star schema to form a snowflake and weigh query vs storage tradeoffs.", ["snowflake", "normalized dimensions", "OLAP", "storage"], False),
    ("Factless Fact Tables", "Model many-to-many event coverage with factless fact tables capturing only keys.", ["factless fact", "coverage", "many-to-many", "event"], False),
    ("Slowly Changing Dimensions (SCD)", "Implement SCD Types 1-4 to track history of dimension attribute changes over time.", ["SCD", "Type 2", "history", "effective dates"], False),
    ("Bridge Tables for Many-to-Many", "Model many-to-many relationships with bridge tables and resolve aggregates correctly.", ["bridge table", "many-to-many", "junction", "aggregate"], False),
    ("Surrogate vs Natural Keys", "Choose between surrogate and natural keys, weighing stability, size, and join performance.", ["surrogate key", "natural key", "stability", "joins"], False),
    ("Composite Primary Keys", "Design composite primary keys and reason about their impact on indexes and foreign keys.", ["composite key", "primary key", "indexes", "foreign key"], False),
    ("Foreign Key Constraints", "Enforce referential integrity with foreign keys and choose RESTRICT, CASCADE, and SET NULL actions.", ["foreign key", "referential integrity", "cascade", "restrict"], False),
    ("CHECK Constraints", "Use CHECK constraints and domains to enforce row-level business rules declaratively.", ["check constraint", "domain", "business rules", "validation"], False),
    ("Triggers (Before/After)", "Implement before- and after-triggers for auditing and derived-column maintenance, noting pitfalls.", ["triggers", "before/after", "audit", "side effects"], False),
    ("Stored Procedures", "Design stored procedures for encapsulated logic and weigh security and maintainability tradeoffs.", ["stored procedures", "encapsulation", "security", "maintainability"], False),
    ("User-Defined Functions", "Build deterministic and volatile SQL functions while understanding inlining and planner effects.", ["UDF", "deterministic", "inlining", "planner"], False),
    ("Views: Materialized vs Virtual", "Compare materialized and virtual views for query abstraction and refresh strategies.", ["views", "materialized", "refresh", "abstraction"], False),
    ("Window Functions", "Use RANK, DENSE_RANK, ROW_NUMBER, and framing clauses for analytic queries.", ["window functions", "rank", "frame", "analytic"], False),
    ("Common Table Expressions", "Refactor complex queries with CTEs for readability and understand materialization behavior.", ["CTE", "readability", "materialization", "recursion"], False),
    ("Recursive CTEs", "Model hierarchies and graph traversals with recursive CTEs using an anchor and a recursive member.", ["recursive CTE", "hierarchy", "traversal", "anchor"], False),
    ("Pivoting and Unpivoting", "Pivot rows to columns and unpivot columns to rows using conditional aggregation and UNION.", ["pivot", "unpivot", "conditional aggregation", "cross tab"], False),
    ("Self-Joins for Hierarchies", "Use self-joins to traverse adjacency lists and compute transitive relationships.", ["self-join", "adjacency list", "hierarchy", "transitive"], False),
    ("Recursive Queries for Trees (Adjacency List)", "Traverse tree-structured adjacency data with recursive queries and termination guards.", ["adjacency list", "recursive query", "tree", "termination"], False),
    ("Nested Sets Model", "Store trees using nested sets with left/right preorder bounds for subtree queries.", ["nested sets", "left/right", "subtree", "preorder"], False),
    ("Materialized Path (ltree)", "Index tree paths with ltree or materialized path strings for prefix queries.", ["materialized path", "ltree", "prefix", "GiST"], False),
    ("Closure Table for Hierarchies", "Store all ancestor-descendant pairs in a closure table for fast descendant and depth queries.", ["closure table", "hierarchy", "ancestor", "descendant"], False),
    ("B-Tree Index Internals", "Explain B-tree index page layout, fan-out, and how range scans traverse leaf links.", ["B-tree index", "fan-out", "leaf links", "range scan"], False),
    ("Hash Index", "Use hash indexes for equality lookups and note their unsuitability for range queries.", ["hash index", "equality", "no range", "buckets"], False),
    ("Bitmap Index", "Apply bitmap indexes to low-cardinality columns and convert rowids in bulk for OLAP workloads.", ["bitmap index", "low cardinality", "OLAP", "rowid"], False),
    ("GiST and GIN Indexes", "Choose GiST vs GIN for full-text and custom data types based on query and update patterns.", ["GiST", "GIN", "full-text", "custom types"], False),
    ("Composite Index Column Order", "Design composite indexes with column order matching equality, sort, and range predicates.", ["composite index", "column order", "selectivity", "range"], False),
    ("Covering Index (INCLUDE)", "Add included non-key columns to make an index covering and enable index-only scans.", ["covering index", "INCLUDE", "index-only scan", "visibility map"], False),
    ("Partial Index (WHERE Clause)", "Create partial indexes on a filtered subset to reduce size and speed common queries.", ["partial index", "WHERE", "subset", "size"], False),
    ("Functional/Expression Index", "Index the result of an expression or function to accelerate transformed predicates.", ["expression index", "function", "immutable", "predicate"], False),
    ("Index-Only Scans and Visibility Map", "Explain how visibility maps enable index-only scans and the cost of vacuuming to maintain them.", ["index-only scan", "visibility map", "vacuum", "heap fetch"], False),
    ("Index Range Scan vs Skip Scan", "Contrast range scans with skip scans that handle leading-column equality filters.", ["range scan", "skip scan", "composite index", "leading column"], False),
    ("Clustered vs Non-Clustered Index", "Distinguish clustered (index-organized) tables from non-clustered secondary indexes.", ["clustered", "non-clustered", "index-organized", "secondary"], False),
    ("B+ Tree Leaf Page Layout", "Describe the leaf page layout of a B+ tree including key order, row pointers, and links.", ["B+ tree", "leaf page", "pointers", "links"], False),
    ("Write-Ahead Logging (WAL)", "Implement WAL semantics so that no data page is flushed before its redo log record.", ["WAL", "redo log", "durability", "flush order"], False),
    ("ARIES Recovery Algorithm", "Explain ARIES analysis, redo, and undo phases for crash recovery with per-page LSNs.", ["ARIES", "analysis", "redo", "undo"], False),
    ("Undo/Redo Logging", "Contrast undo-only, redo-only, and undo-redo logging with respect to steal and no-force policies.", ["undo", "redo", "steal", "no-force"], False),
    ("Checkpointing Strategies", "Design fuzzy checkpointing to bound recovery time while minimizing foreground pauses.", ["checkpoint", "fuzzy", "recovery time", "LSN"], False),
    ("Two-Phase Commit (2PC)", "Coordinate a transaction across nodes with a prepare-then-commit protocol and a coordinator log.", ["2PC", "prepare", "commit", "coordinator"], False),
    ("Three-Phase Commit (3PC)", "Add a pre-commit phase to 2PC to reduce blocking on coordinator failure under assumptions.", ["3PC", "pre-commit", "non-blocking", "timing"], False),
    ("Paxos Consensus", "Implement single-decree Paxos with proposers, acceptors, and learners achieving safety under quorum.", ["Paxos", "proposer", "acceptor", "quorum"], False),
    ("Raft Consensus", "Implement Raft leader election, log replication, and safety via term-based commit indices.", ["Raft", "leader election", "log replication", "term"], False),
    ("Multi-Paxos", "Optimize Paxos to a steady-state leader batching many instances over a stable leader.", ["Multi-Paxos", "leader", "batching", "steady state"], False),
    ("Snapshot Isolation", "Provide snapshot isolation using transaction start timestamps and version chains to avoid read locks.", ["snapshot isolation", "version chain", "timestamp", "no read lock"], False),
    ("Serializable Snapshot Isolation (SSI)", "Detect dangerous read/write patterns to provide serializability over snapshot isolation.", ["SSI", "serializable", "conflict", "safe retry"], False),
    ("MVCC Implementation", "Build multi-version concurrency control with tuple xmin/xmax and visibility checks.", ["MVCC", "xmin/xmax", "version chain", "visibility"], False),
    ("Two-Phase Locking (2PL)", "Implement strict two-phase locking growing then shrinking lock phases to guarantee serializability.", ["2PL", "growing", "shrinking", "strict"], False),
    ("Deadlock Detection in DB", "Use a wait-for graph to detect and resolve deadlocks among transactions.", ["deadlock", "wait-for graph", "detection", "victim"], False),
    ("Gap Locks and Next-Key Locking", "Prevent phantom reads with gap and next-key locking in InnoDB repeatable-read.", ["gap lock", "next-key", "phantom", "InnoDB"], False),
    ("Optimistic Concurrency Control", "Validate transactions at commit time using read and write sets instead of locks.", ["OCC", "validation", "read/write set", "conflict"], False),
    ("Isolation Levels", "Contrast read-uncommitted, read-committed, repeatable-read, and serializable isolation.", ["isolation", "read committed", "repeatable read", "serializable"], False),
    ("Phantom Read Prevention", "Prevent phantom reads via predicate locking or gap locks to keep a predicate result set stable.", ["phantom", "predicate lock", "gap lock", "stability"], False),
    ("Distributed Transactions (XA)", "Coordinate distributed XA transactions across resource managers with prepare/commit phases.", ["XA", "distributed", "prepare", "resource manager"], False),
    ("Sagas (Long-Running Transactions)", "Model long-running business transactions as a saga of compensating local actions.", ["saga", "compensation", "long-running", "choreography"], False),
    ("Outbox Pattern", "Reliably publish events to a broker by writing them transactionally to an outbox table.", ["outbox", "exactly-once", "event publishing", "transactional"], False),
    ("Change Data Capture (CDC)", "Stream row changes from a database by reading the WAL or transaction log.", ["CDC", "WAL", "streaming", "transaction log"], False),
    ("CQRS", "Separate command (write) and query (read) models to optimize each independently.", ["CQRS", "command", "query", "separation"], False),
    ("Event Sourcing", "Store domain events as the source of truth and project read models from the event log.", ["event sourcing", "events", "projection", "replay"], False),
    ("Sharding with Consistent Hashing", "Distribute rows across shards using consistent hashing to minimize reshuffle on resharding.", ["sharding", "consistent hashing", "virtual nodes", "reshuffle"], False),
    ("Partition Pruning and Partition-Wise Join", "Use declarative partitioning to prune scans and co-locate partitions for joins.", ["partitioning", "pruning", "partition-wise join", "range/list"], False),
]

CATEGORIES.append(("SQL & Database Design", 221, sql))


# ---------------------------------------------------------------------------
# 5. System Design  (ids 286-360, 75 questions)
# ---------------------------------------------------------------------------
system_design = [
    ("Design a URL Shortener", "Design a service that maps long URLs to short codes with high read throughput and analytics.", ["URL shortener", "base62", "sharding", "cache"], False),
    ("Design Pastebin", "Design a paste-sharing service handling large text blobs, expiration, and rate limits.", ["pastebin", "blob storage", "expiration", "rate limit"], False),
    ("Design a Key-Value Store", "Design a distributed key-value store with consistent hashing, replication, and tunable consistency.", ["key-value store", "consistent hashing", "replication", "quorum"], False),
    ("Design a Distributed Cache", "Design a Memcached/Redis-like cache with eviction, sharding, and failover.", ["cache", "eviction", "sharding", "failover"], False),
    ("Design a Rate Limiter (Token Bucket)", "Design a token-bucket rate limiter distributed across nodes with sliding accuracy.", ["rate limiter", "token bucket", "distributed", "sliding"], False),
    ("Design a Rate Limiter (Sliding Window)", "Implement a sliding-window rate limiter using sorted sets or a rolling counter sketch.", ["rate limiter", "sliding window", "sorted set", "accuracy"], False),
    ("Design Twitter / News Feed", "Design a timeline service deciding between fan-out on write and fan-out on read.", ["news feed", "fan-out", "timeline", "ranking"], False),
    ("Design Twitter Timeline Fan-Out", "Compare push-on-write vs pull-on-read fan-out for celebrity and normal users.", ["fan-out", "push", "pull", "celebrity"], False),
    ("Design Instagram / Photo Sharing", "Design a photo-sharing service with object storage, CDN, and metadata sharding.", ["photo sharing", "object storage", "CDN", "metadata"], False),
    ("Design Dropbox / File Storage", "Design a file-sync service with chunking, deduplication, and conflict resolution.", ["file sync", "chunking", "dedup", "conflict"], False),
    ("Design Google Drive Collaborative Editing", "Design real-time collaborative editing using operational transformation or CRDTs.", ["collaborative editing", "OT", "CRDT", "conflict"], False),
    ("Design YouTube / Video Streaming", "Design a video platform with adaptive bitrate, transcoding pipelines, and CDN.", ["video streaming", "adaptive bitrate", "transcoding", "CDN"], False),
    ("Design Netflix / CDN Streaming", "Design a streaming CDN with origin shields, caching tiers, and ABR playback.", ["streaming CDN", "origin shield", "cache tier", "ABR"], False),
    ("Design a Chat System (WhatsApp)", "Design a real-time chat service with presence, delivery receipts, and offline sync.", ["chat", "presence", "delivery receipts", "offline sync"], False),
    ("Design a Presence Service", "Design a presence service tracking online status with heartbeats and a pub/sub fan-out.", ["presence", "heartbeat", "pub/sub", "fan-out"], False),
    ("Design a Notification Service", "Design a multi-channel notification fan-out with dedup, batching, and user preferences.", ["notifications", "fan-out", "dedup", "preferences"], False),
    ("Design a Search Engine", "Design a search engine with crawling, indexing, ranking, and query serving.", ["search", "crawling", "inverted index", "ranking"], False),
    ("Design a Web Crawler", "Design a distributed web crawler with URL frontier scheduling, politeness, and dedup.", ["crawler", "frontier", "politeness", "dedup"], False),
    ("Design Google Maps / Routing", " Design a routing service using contraction hierarchies and A* on a road graph.", ["routing", "A*", "contraction hierarchies", "road graph"], False),
    ("Design a Ride-Sharing System (Uber)", "Design ride matching with geospatial indexing, surge pricing, and dispatch.", ["ride sharing", "geospatial", "dispatch", "surge"], False),
    ("Design Geospatial Indexing (Geohash/Quadtree)", "Index moving vehicles by geohash or quadtree for nearby-driver queries.", ["geospatial", "geohash", "quadtree", "nearby"], False),
    ("Design a Ticket Booking System", "Design a seat-booking service with hold locks, idempotency, and overbooking prevention.", ["booking", "seat hold", "idempotency", "overbooking"], False),
    ("Design an Event Ticketing System", "Design high-contention flash-sale ticketing with virtual queues and inventory sharding.", ["flash sale", "virtual queue", "inventory", "contention"], False),
    ("Design an E-Commerce Checkout", "Design a checkout pipeline with cart, pricing, inventory, and payment orchestration.", ["checkout", "cart", "pricing", "payment"], False),
    ("Design an Inventory Service", "Design a per-SKU inventory service with strong consistency and reservation semantics.", ["inventory", "reservation", "strong consistency", "SKU"], False),
    ("Design a Payment System", "Design a payment processing system with idempotency, ledgering, and reconciliation.", ["payments", "ledger", "idempotency", "reconciliation"], False),
    ("Design Idempotent Payment Processing", "Ensure payment APIs are idempotent using idempotency keys and a dedup store.", ["idempotency", "payment", "dedup", "ledger"], False),
    ("Design a Distributed Counter (Likes)", "Design an eventually consistent like counter using CRDTs and sharded counts.", ["counter", "CRDT", "sharded", "eventual"], False),
    ("Design a Leaderboard (Sorted Sets)", "Design a global leaderboard using Redis sorted sets with sharded rollups.", ["leaderboard", "sorted sets", "sharding", "rollup"], False),
    ("Design a Comment System", "Design a threaded comment system with materialized paths and pagination.", ["comments", "threading", "materialized path", "pagination"], False),
    ("Design an Analytics/Event Pipeline", "Design an event ingestion pipeline with Kafka, stream processing, and warehousing.", ["analytics", "Kafka", "stream processing", "warehouse"], False),
    ("Design a Metrics/Monitoring System", "Design a time-series metrics system with cardinality control and downsampling.", ["metrics", "time-series", "cardinality", "downsampling"], False),
    ("Design a Log Aggregation System", "Design a log pipeline with ingestion, indexing, retention, and query.", ["logs", "ingestion", "indexing", "retention"], False),
    ("Design a Distributed Job Scheduler", "Design a fault-tolerant scheduler with leases, retries, and idempotent execution.", ["scheduler", "leases", "retries", "idempotent"], False),
    ("Design Cron-as-a-Service", "Design a multi-tenant cron service distributing timed jobs across workers.", ["cron", "multi-tenant", "scheduling", "workers"], False),
    ("Design a Message Queue (Kafka)", "Design a partitioned, replicated log-based message queue with consumer groups.", ["message queue", "Kafka", "partition", "replication"], False),
    ("Design Kafka Log Structure", "Explain Kafka's append-only segmented log with indexes and retention by size/time.", ["Kafka log", "segments", "index", "retention"], False),
    ("Design Consumer Groups and Rebalancing", "Design consumer group coordination with rebalancing strategies and session timeouts.", ["consumer group", "rebalancing", "coordinator", "session"], False),
    ("Design Exactly-Once Delivery Semantics", "Achieve exactly-once delivery using idempotent producers and transactional consumption.", ["exactly-once", "idempotent producer", "transactions", "EOS"], False),
    ("Design a Pub/Sub System", "Design a topic-based pub/sub with durable subscriptions and backpressure.", ["pub/sub", "topics", "durable", "backpressure"], False),
    ("Design a Workflow Engine", "Design a durable workflow engine (Temporal/Airflow-like) with retries, timers, and state.", ["workflow", "durable", "retries", "timers"], False),
    ("Design an Email Service", "Design a transactional email service with provider failover and bounce handling.", ["email", "provider failover", "bounce", "throttling"], False),
    ("Design a DNS System", "Design a hierarchical, cached DNS resolver with TTLs and negative caching.", ["DNS", "hierarchy", "TTL", "negative caching"], False),
    ("Design a CDN", "Design a CDN with edge caches, origin pull, and cache invalidation strategies.", ["CDN", "edge cache", "origin pull", "invalidation"], False),
    ("Design a Layer-4 Load Balancer", "Design an L4 load balancer using connection tracking and consistent hashing.", ["L4 LB", "connection tracking", "consistent hashing", "NAT/DSR"], False),
    ("Design a Layer-7 Load Balancer", "Design an L7 load balancer with content-based routing, TLS termination, and retries.", ["L7 LB", "content routing", "TLS termination", "retries"], False),
    ("Design Consistent Hashing", "Implement consistent hashing with virtual nodes for balanced, low-reshuffle distribution.", ["consistent hashing", "virtual nodes", "ring", "reshuffle"], False),
    ("Design Virtual Nodes (Vnodes)", "Add virtual nodes to consistent hashing to smooth load and reduce hotspots.", ["vnodes", "load balancing", "hotspots", "ring"], False),
    ("Design Service Registry and Discovery", "Design a service registry with health checks and client-side discovery.", ["service discovery", "registry", "health checks", "client-side"], False),
    ("Design an API Gateway", "Design an API gateway with auth, rate limiting, routing, and observability.", ["API gateway", "auth", "rate limit", "routing"], False),
    ("Design a BFF (Backend for Frontend)", "Design a backend-for-frontend layer that aggregates services for a specific client.", ["BFF", "aggregation", "client-specific", "edge"], False),
    ("Design a Circuit Breaker", "Implement a circuit breaker with half-open probing and configurable thresholds.", ["circuit breaker", "half-open", "thresholds", "resilience"], False),
    ("Design a Bulkhead Pattern", "Isolate failure domains with bulkheads limiting concurrency per dependency.", ["bulkhead", "isolation", "concurrency limit", "failure domain"], False),
    ("Design Retry with Backoff and Jitter", "Design retries with exponential backoff, jitter, and deadline propagation.", ["retry", "exponential backoff", "jitter", "deadline"], False),
    ("Design a Feature Flag Service", "Design a feature flag service with targeting rules and live config updates.", ["feature flags", "targeting", "live config", "rollouts"], False),
    ("Design a Config Service", "Design a dynamic configuration service with watch notifications and versioning.", ["config", "dynamic", "watch", "versioning"], False),
    ("Design a Secret Management Service", "Design a Vault-like secret store with encryption, leasing, and audit logging.", ["secrets", "encryption", "leasing", "audit"], False),
    ("Design Multi-Region Active-Active", "Design an active-active multi-region system handling conflict resolution and routing.", ["active-active", "multi-region", "conflict", "routing"], False),
    ("Design Multi-Region Active-Passive", "Design an active-passive multi-region system with failover and data replication.", ["active-passive", "failover", "replication", "RTO"], False),
    ("Design Disaster Recovery (RPO/RTO)", "Quantify RPO/RTO and design backup, replication, and failover to meet them.", ["DR", "RPO", "RTO", "failover"], False),
    ("Design a Deduplication Service", "Design a service that deduplicates events using content hashing and a windowed store.", ["dedup", "content hash", "windowed", "idempotency"], False),
    ("Design a Distributed Lock Service", "Design a distributed lock with fencing tokens and lease renewal.", ["distributed lock", "fencing token", "lease", "renewal"], False),
    ("Design a Consensus-Based Config Store", "Design an etcd/ZooKeeper-like config store using Raft and watchers.", ["config store", "Raft", "watchers", "linearizable"], False),
    ("Design Vector Clocks / Logical Clocks", "Use vector clocks to detect concurrent updates in a distributed store.", ["vector clock", "logical clock", "concurrency", "causality"], False),
    ("Design Hybrid Logical Clocks (HLC)", "Combine physical and logical time into HLCs for bounded drift ordering.", ["HLC", "physical", "logical", "drift"], False),
    ("Design a Quorum System", "Design tunable R/W quorums trading consistency for latency and availability.", ["quorum", "R/W", "consistency", "latency"], False),
    ("Design Read-Repair and Anti-Entropy", "Repair divergent replicas via read-repair and background anti-entropy.", ["read repair", "anti-entropy", "replica", "consistency"], False),
    ("Design Hinted Handoff (Dynamo)", "Store writes for temporarily unavailable replicas and hand them off on recovery.", ["hinted handoff", "Dynamo", "replica", "recovery"], False),
    ("Design a Merkle Tree for Anti-Entropy", "Compare replicas with Merkle trees to localize and repair differences.", ["Merkle tree", "anti-entropy", "replica diff", "repair"], False),
    ("Design a Gossip Protocol", "Propagate membership and state updates across nodes with a bounded gossip round.", ["gossip", "membership", "epidemic", "bounded"], False),
    ("Design a Distributed Tracing System", "Design an OpenTelemetry-style tracing system with spans, sampling, and context propagation.", ["tracing", "spans", "sampling", "context"], False),
    ("Design Context Propagation (W3C Trace Context)", "Propagate trace context across process boundaries with W3C headers.", ["trace context", "W3C", "propagation", "headers"], False),
    ("Design an A/B Testing Platform", "Design an experimentation platform with bucketing, metrics, and statistical guardrails.", ["A/B testing", "bucketing", "metrics", "significance"], False),
    ("Design a Recommendation System", "Design a collaborative filtering recommender with embedding retrieval and ranking.", ["recommendations", "collaborative filtering", "embeddings", "ranking"], False),
    ("Design an ML Feature Store", "Design a feature store serving consistent features online and offline.", ["feature store", "online/offline", "consistency", "serving"], False),
]

CATEGORIES.append(("System Design", 286, system_design))


# ---------------------------------------------------------------------------
# 6. Memory Management  (ids 361-410, 50 questions)
# ---------------------------------------------------------------------------
memory = [
    ("Slab Allocator", "Implement a slab allocator caching fixed-size object states for the kernel to reduce fragmentation.", ["slab", "caches", "fixed-size", "kernel"], False),
    ("Slub Allocator", "Explain the SLUB allocator's simpler, per-CPU design replacing the classic slab allocator.", ["SLUB", "per-CPU", "freelist", "kernel"], False),
    ("Buddy Allocator", "Implement a binary buddy allocator splitting and coalescing power-of-two blocks.", ["buddy", "coalescing", "power-of-two", "split"], False),
    ("Arena Allocator", "Build an arena allocator that bulk-allocates from a parent and frees all at once.", ["arena", "bulk alloc", "reset", "region"], False),
    ("Region/Arena Bump Allocator", "Implement a bump pointer allocator within a region for O(1) allocation and bulk free.", ["bump allocator", "region", "O(1)", "bulk free"], False),
    ("Pool Allocator (Fixed-Size)", "Implement a fixed-size pool allocator using a free list for constant-time alloc/free.", ["pool", "fixed-size", "free list", "O(1)"], False),
    ("Stack (Linear) Allocator", "Implement a stack allocator with markers to roll back to a previous top.", ["stack allocator", "markers", "LIFO free", "linear"], False),
    ("jemalloc Design", "Explain jemalloc's size-class bins, arenas, and thread caches for low fragmentation.", ["jemalloc", "size class", "arena", "thread cache"], False),
    ("tcmalloc Design", "Explain tcmalloc's thread-local caches and span-based central heap for scalable allocation.", ["tcmalloc", "thread-local", "spans", "central heap"], False),
    ("mimalloc Design", "Explain mimalloc's per-CPU sharded free lists and delayed resets for high throughput.", ["mimalloc", "per-CPU", "free lists", "delayed reset"], False),
    ("Thread-Local Caches (TCache)", "Design thread-local allocation caches with periodic return to the global heap.", ["TCache", "thread-local", "global heap", "scavenge"], False),
    ("Mark-and-Sweep GC", "Implement a mark-and-sweep collector that traces live objects and sweeps dead ones.", ["mark-and-sweep", "tracing", "roots", "sweep"], False),
    ("Mark-Compact GC", "Implement a mark-compact collector that eliminates fragmentation by sliding live objects.", ["mark-compact", "compaction", "fragmentation", "forwarding"], False),
    ("Copying GC (Semispace)", "Implement a copying collector that evacuates live objects between two semispaces.", ["copying GC", "semispace", "evacuation", "forwarding"], False),
    ("Generational GC", "Design a generational collector with young and old generations using remembered sets.", ["generational", "young/old", "remembered set", "promotion"], False),
    ("G1 GC", "Explain the Garbage-First collector's region-based layout and pause-time predictability.", ["G1", "regions", "pause prediction", "compaction"], False),
    ("ZGC (Low Latency)", "Explain ZGC's colored pointers and load barriers for sub-millisecond pauses.", ["ZGC", "colored pointer", "load barrier", "low latency"], False),
    ("Shenandoah GC", "Explain Shenandoah's concurrent evacuation using Brooks forwarding pointers.", ["Shenandoah", "Brooks pointer", "concurrent evacuation", "low latency"], False),
    ("Concurrent Marking", "Implement concurrent marking with safe-points and handshakes to avoid long pauses.", ["concurrent marking", "safe-point", "handshake", "pause"], False),
    ("Tri-Color Marking", "Implement the tri-color invariant (white, gray, black) during tracing.", ["tri-color", "white/gray/black", "invariant", "tracing"], False),
    ("Write Barriers (SATB/INC)", "Implement SATB and incremental-update write barriers to maintain tri-color invariance.", ["write barrier", "SATB", "incremental", "invariant"], False),
    ("Reference Counting", "Implement reference counting with cycle detection to reclaim unreachable cycles.", ["reference counting", "cycles", "weak refs", "deferred"], False),
    ("Tracing vs Reference Counting", "Contrast tracing and reference counting collectors on pause time and throughput.", ["tracing", "reference counting", "pause", "throughput"], False),
    ("Memory Pools and Object Pools", "Design object pools to amortize allocation of expensive-to-construct objects.", ["object pool", "amortize", "construct cost", "reuse"], False),
    ("Hazard Pointers for Reclamation", "Use hazard pointers to safely reclaim memory in lock-free data structures.", ["hazard pointer", "reclamation", "lock-free", "ABA"], False),
    ("Epoch-Based Reclamation (EBR)", "Defer reclamation until epochs advance past all readers for lock-free safety.", ["EBR", "epoch", "deferred", "lock-free"], False),
    ("RCU-Based Memory Reclamation", "Reclaim nodes after a grace period so concurrent readers see consistent state.", ["RCU", "grace period", "reclamation", "readers"], False),
    ("Quiescent-State-Based Reclamation", "Reclaim memory at quiescent states observed across threads for lock-free safety.", ["quiescent state", "reclamation", "lock-free", "safety"], False),
    ("Deferred Reference Counting", "Batch reference-count updates to amortize their cost on the hot path.", ["deferred RC", "batching", "amortize", "hot path"], False),
    ("Virtual Memory and Paging", "Implement paging that maps virtual to physical pages via page tables.", ["virtual memory", "paging", "page table", "translation"], False),
    ("Multi-Level Page Tables", "Design multi-level page tables to compactly represent sparse virtual address spaces.", ["multi-level page table", "sparse", "compact", "translation"], False),
    ("TLB and TLB Shootdown", "Explain the TLB cache of translations and the cost of cross-CPU shootdowns.", ["TLB", "shootdown", "IPI", "translation cache"], False),
    ("Huge Pages (Transparent)", "Use huge pages to reduce TLB pressure and page-walk cost for large allocations.", ["huge pages", "THP", "TLB", "page walk"], False),
    ("mmap and Virtual Address Space", "Use mmap to map files and anonymous memory into the process address space.", ["mmap", "virtual address", "anonymous", "file-backed"], False),
    ("Copy-on-Write (CoW) Pages", "Share read-only pages and copy only on write to enable cheap fork and sharing.", ["copy-on-write", "fork", "sharing", "page protection"], False),
    ("Demand Paging", "Load pages on first access via page faults to avoid eager allocation.", ["demand paging", "page fault", "lazy", "zero fill"], False),
    ("Page Replacement (LRU/Clock)", "Implement LRU and clock page replacement policies for finite physical memory.", ["page replacement", "LRU", "clock", "eviction"], False),
    ("ARC Page Replacement", "Implement the adaptive replacement cache policy balancing recency and frequency.", ["ARC", "adaptive", "recency", "frequency"], False),
    ("Working Set and RSS", "Estimate the working set size and resident set to size memory and detect thrashing.", ["working set", "RSS", "thrashing", "sizing"], False),
    ("Memory-Mapped Files", "Access files through mapped pages for zero-copy I/O and kernel-managed caching.", ["mmap files", "zero-copy", "page cache", "I/O"], False),
    ("OOM Killer", "Design an out-of-memory killer that selects victims based on memory score.", ["OOM", "killer", "victim selection", "memory score"], False),
    ("Overcommit and OOM", "Reason about memory overcommit, commit limits, and the consequences for OOM.", ["overcommit", "commit limit", "OOM", "accounting"], False),
    ("Memory Fragmentation (External/Internal)", "Distinguish external and internal fragmentation and mitigate each.", ["fragmentation", "external", "internal", "coalescing"], False),
    ("Compaction", "Compact the heap to reduce external fragmentation via forwarding addresses.", ["compaction", "forwarding", "fragmentation", "heap"], False),
    ("Alignment and Padding", "Lay out structs with alignment rules to avoid misaligned access and padding waste.", ["alignment", "padding", "struct layout", "ABI"], False),
    ("Struct Packing and Cache Lines", "Pack structs to fit within cache lines and trade size against access speed.", ["packing", "cache line", "layout", "alignment"], False),
    ("False Sharing", "Diagnose false sharing on shared mutable fields in the same cache line and pad them apart.", ["false sharing", "cache line", "padding", "contention"], False),
    ("Cache-Friendly Data Layout (SoA vs AoS)", "Choose between array-of-structs and struct-of-arrays for SIMD and cache efficiency.", ["SoA", "AoS", "SIMD", "cache"], False),
    ("NUMA Allocation Policies", "Apply NUMA-aware allocation and first-touch to keep memory local to compute.", ["NUMA", "first-touch", "locality", "allocation"], False),
    ("Huge Page Table Walks", "Analyze page-walk cost with huge pages and the resulting TLB savings.", ["page walk", "huge page", "TLB", "cost"], False),
]

CATEGORIES.append(("Memory Management", 361, memory))


# ---------------------------------------------------------------------------
# 7. Performance & Profiling  (ids 411-460, 50 questions)
# ---------------------------------------------------------------------------
performance = [
    ("CPU Profiling: Sampling vs Instrumentation", "Contrast sampling and instrumentation profilers for accuracy vs overhead.", ["profiling", "sampling", "instrumentation", "overhead"], False),
    ("Flame Graphs", "Build and read flame graphs to identify hot code paths from stack samples.", ["flame graph", "stack samples", "hot path", "visualization"], False),
    ("perf / perf_events", "Use perf to capture hardware counters, branch misses, and cache misses.", ["perf", "perf_events", "PMU", "counters"], False),
    ("eBPF for Tracing", "Write eBPF probes to trace kernel and user functions with low overhead.", ["eBPF", "tracing", "kprobes", "uprobes"], False),
    ("strace / ltrace", "Use strace and ltrace to attribute time spent in system and library calls.", ["strace", "ltrace", "syscall", "library"], False),
    ("gprof and the GNU Profiler", "Use gprof call-graph and flat profiles to locate hot functions in C programs.", ["gprof", "call graph", "flat profile", "instrumentation"], False),
    ("Branch Prediction and Misprediction", "Reason about branch prediction cost and restructure code to be branch-predictable.", ["branch prediction", "misprediction", "pipeline", "branchless"], False),
    ("CPU Pipeline Stalls", "Identify pipeline stalls from data hazards and long-latency instructions.", ["pipeline", "stall", "data hazard", "latency"], False),
    ("Superscalar and Out-of-Order Execution", "Explain how superscalar and out-of-order execution expose instruction-level parallelism.", ["superscalar", "OoO", "ILP", "renaming"], False),
    ("Cache Hierarchy (L1/L2/L3)", "Model the L1/L2/L3 cache hierarchy and quantify latency and bandwidth at each level.", ["cache", "L1/L2/L3", "latency", "bandwidth"], False),
    ("Cache Lines and Prefetching", "Size data accesses to cache lines and exploit hardware prefetching.", ["cache line", "prefetch", "locality", "size"], False),
    ("Software Prefetching", "Insert software prefetch instructions to hide memory latency in streaming loops.", ["software prefetch", "latency hiding", "streaming", "intrinsics"], False),
    ("Data-Oriented Design", "Restructure data for cache efficiency by separating hot and cold fields.", ["DOD", "hot/cold split", "cache efficiency", "layout"], False),
    ("SIMD Vectorization", "Vectorize loops with SIMD intrinsics to process multiple elements per instruction.", ["SIMD", "vectorization", "intrinsics", "lanes"], False),
    ("Auto-Vectorization", "Write loops that the compiler auto-vectorizes and verify with assembly inspection.", ["auto-vectorization", "compiler", "restrict", "alignment"], False),
    ("Loop Unrolling", "Unroll loops to reduce loop overhead and expose ILP, balancing code-size costs.", ["unrolling", "ILP", "loop overhead", "code size"], False),
    ("Loop Tiling / Blocking", "Tile nested loops to fit working sets in cache for matrix computations.", ["tiling", "blocking", "cache", "matrix"], False),
    ("Cache-Oblivious Algorithms", "Design algorithms that achieve cache efficiency without tuning to cache size.", ["cache-oblivious", "recursive blocking", "portable", "I/O"], False),
    ("Branch-Free Code (Bit Hacks)", "Replace branches with bit manipulation to avoid mispredictions on data-dependent paths.", ["bit hacks", "branchless", "misprediction", "flags"], False),
    ("Branchless Binary Search", "Implement a branchless binary search using conditional moves or index arithmetic.", ["branchless", "binary search", "conditional move", "Eytzinger"], False),
    ("Latency Numbers Every Programmer Should Know", "Reason about L1, DRAM, SSD, and network latencies to design fast systems.", ["latency", "L1", "DRAM", "network"], False),
    ("Memory Bandwidth and Streams", "Measure memory bandwidth with the STREAM benchmark and detect bandwidth-bound code.", ["bandwidth", "STREAM", "memory-bound", "measurement"], False),
    ("NUMA Effects on Performance", "Diagnose NUMA-local vs remote access penalties for multi-socket workloads.", ["NUMA", "remote access", "locality", "sockets"], False),
    ("Hyperthreading/SMT Throughput", "Reason about SMT throughput gains and contention on shared execution resources.", ["SMT", "hyperthreading", "throughput", "contention"], False),
    ("Memory-Bound vs Compute-Bound", "Classify code as memory- or compute-bound to pick the right optimization.", ["memory-bound", "compute-bound", "roofline", "classification"], False),
    ("Roofline Model", "Plot the roofline model to see whether arithmetic intensity caps performance.", ["roofline", "arithmetic intensity", "peak", "bandwidth"], False),
    ("Amdahl's Law", "Apply Amdahl's law to bound speedup from parallelizing a fraction of a program.", ["Amdahl", "speedup", "parallel fraction", "bounds"], False),
    ("Gustafson's Law", "Apply Gustafson's law to scale problems with processors rather than fix them.", ["Gustafson", "scaled speedup", "parallel", "problem size"], False),
    ("Tail Latency (P99)", "Measure and reduce P99 latency by eliminating stragglers in fan-out services.", ["tail latency", "P99", "straggler", "fan-out"], False),
    ("Load Testing (Throughput/Latency)", "Design load tests that report throughput-latency curves under sustained load.", ["load testing", "throughput", "latency", "sustained"], False),
    ("Little's Law", "Apply Little's Law (L = lambda * W) to size queues and concurrency.", ["Little's Law", "concurrency", "queue", "throughput"], False),
    ("Queueing Theory (M/M/1)", "Model an M/M/1 queue to predict latency under arrival and service rate variation.", ["queueing theory", "M/M/1", "utilization", "latency"], False),
    ("Backpressure and Load Shedding", "Apply backpressure and load shedding to preserve latency under overload.", ["backpressure", "load shedding", "overload", "latency"], False),
    ("Circuit Breakers for Performance", "Use circuit breakers to fail fast and protect latency during dependency outages.", ["circuit breaker", "fail fast", "latency", "outage"], False),
    ("Async I/O (epoll/io_uring)", "Use epoll and io_uring to drive many I/O operations per thread.", ["async I/O", "epoll", "io_uring", "scalability"], False),
    ("io_uring vs epoll", "Contrast io_uring's submission/completion queues with epoll's readiness model.", ["io_uring", "epoll", "submission queue", "completion"], False),
    ("Zero-Copy I/O (sendfile)", "Use sendfile and splice to move data between file descriptors without copying.", ["zero-copy", "sendfile", "splice", "kernel buffer"], False),
    ("Scatter-Gather I/O", "Use scatter-gather (readv/writev) to coalesce multiple buffers per syscall.", ["scatter-gather", "readv", "writev", "syscall"], False),
    ("Buffer Batching and Coalescing", "Batch and coalesce small writes into larger buffers to amortize syscall cost.", ["batching", "coalescing", "syscall", "amortize"], False),
    ("I/O Scheduler (deadline/CFQ)", "Explain disk I/O schedulers and their effect on latency and throughput.", ["I/O scheduler", "deadline", "CFQ", "merging"], False),
    ("Direct I/O vs Buffered I/O", "Contrast O_DIRECT with buffered I/O and the page-cache implications.", ["direct I/O", "buffered I/O", "page cache", "O_DIRECT"], False),
    ("Memory-Mapped I/O Performance", "Evaluate mmap-backed I/O performance and TLB/page-fault tradeoffs.", ["mmap", "TLB", "page fault", "performance"], False),
    ("Lock Contention Profiling", "Profile lock contention to find contended locks and convert them to sharded or lock-free forms.", ["lock contention", "profiling", "sharding", "lock-free"], False),
    ("Lock-Free Throughput Analysis", "Analyze whether lock-free structures actually improve throughput under contention.", ["lock-free", "throughput", "contention", "CAS"], False),
    ("False Sharing Mitigation", "Pad shared mutable variables to cache lines to eliminate false sharing.", ["false sharing", "padding", "cache line", "mitigation"], False),
    ("Cache Miss Profiling", "Measure L1/L2/L3 and LLC misses to find cache-unfriendly data layouts.", ["cache miss", "profiling", "PMC", "layout"], False),
    ("TLB Miss Analysis", "Measure TLB misses and mitigate them with huge pages and access locality.", ["TLB miss", "huge pages", "locality", "PMC"], False),
    ("JIT vs Interpreter Performance", "Compare JIT compilation with interpretation and where each pays off.", ["JIT", "interpreter", "compilation", "warmup"], False),
    ("Escape Analysis and Stack Allocation", "Use escape analysis to allocate heap objects on the stack for zero-cost lifetimes.", ["escape analysis", "stack allocation", "GC", "lifetime"], False),
    ("GC Pause Tuning", "Tune a generational collector's heap sizes and barriers to reduce pause times.", ["GC tuning", "pauses", "generational", "heap sizing"], False),
]

CATEGORIES.append(("Performance & Profiling", 411, performance))


# ---------------------------------------------------------------------------
# 8. Security  (ids 461-510, 50 questions)
# ---------------------------------------------------------------------------
security = [
    ("SQL Injection Prevention", "Prevent SQL injection using parameterized queries and strict input validation.", ["SQL injection", "parameterized", "validation", "ORM"], False),
    ("XSS Prevention (CSP)", "Prevent cross-site scripting with output encoding and a Content Security Policy.", ["XSS", "encoding", "CSP", "sanitization"], False),
    ("CSRF Tokens", "Prevent cross-site request forgery with anti-CSRF tokens and SameSite cookies.", ["CSRF", "tokens", "SameSite", "origin"], False),
    ("SSRF Prevention", "Prevent server-side request forgery by validating and restricting outbound destinations.", ["SSRF", "allowlist", "egress", "metadata"], False),
    ("XXE Prevention", "Prevent XML external entity attacks by disabling DTDs and external entity resolution.", ["XXE", "DTD", "external entity", "parser"], False),
    ("Command Injection", "Prevent command injection by avoiding shell calls and using argument arrays.", ["command injection", "shell", "argument array", "escaping"], False),
    ("Path Traversal", "Prevent path traversal by canonicalizing and confining file access to a root.", ["path traversal", "canonicalization", "sandbox", "root"], False),
    ("Insecure Deserialization", "Prevent insecure deserialization by avoiding native formats and enforcing type allowlists.", ["deserialization", "type allowlist", "gadget", "RCE"], False),
    ("OWASP Top 10", "Map a system's defenses to the OWASP Top 10 risk categories.", ["OWASP", "Top 10", "risk", "mitigation"], False),
    ("Password Hashing: bcrypt/scrypt/argon2", "Choose and configure bcrypt, scrypt, and argon2 to slow brute force attacks.", ["password hashing", "bcrypt", "scrypt", "argon2"], False),
    ("Password Salting and Pepping", "Use per-password salts and a server-side pepper to defeat precomputed attacks.", ["salt", "pepper", "precomputed", "rainbow"], False),
    ("JWT Design and Pitfalls", "Design JWTs securely, avoiding alg=none, weak keys, and missing expiry validation.", ["JWT", "alg", "expiry", "signature"], False),
    ("OAuth 2.0 Flows", "Implement authorization-code, client-credentials, and PKCE flows appropriately.", ["OAuth", "authorization code", "PKCE", "client credentials"], False),
    ("OpenID Connect (OIDC)", "Layer OpenID Connect on OAuth 2.0 for authenticated identity via ID tokens.", ["OIDC", "ID token", "OAuth", "identity"], False),
    ("SAML SSO", "Implement SAML single sign-on with signed assertions and the POST redirect binding.", ["SAML", "assertion", "SSO", "binding"], False),
    ("Session Management (Secure Cookies)", "Manage sessions with HttpOnly, Secure, and SameSite cookie attributes.", ["session", "HttpOnly", "Secure", "SameSite"], False),
    ("Refresh Token Rotation", "Rotate refresh tokens on use with reuse detection to limit token theft impact.", ["refresh token", "rotation", "reuse detection", "theft"], False),
    ("MFA / TOTP", "Implement time-based one-time passwords (RFC 6238) for multi-factor authentication.", ["MFA", "TOTP", "RFC 6238", "HMAC"], False),
    ("Passkeys (WebAuthn)", "Implement passkeys using WebAuthn with authenticator attestation and challenge-response.", ["passkeys", "WebAuthn", "attestation", "challenge"], False),
    ("HMAC (Hash-Based MAC)", "Implement HMAC for message authentication using a keyed hash construction.", ["HMAC", "keyed hash", "integrity", "authentication"], False),
    ("AEAD (AES-GCM, ChaCha20-Poly1305)", "Use authenticated encryption with associated data to provide confidentiality and integrity.", ["AEAD", "AES-GCM", "ChaCha20-Poly1305", "nonce"], False),
    ("Public-Key Encryption (RSA-OAEP)", "Encrypt with RSA-OAEP padding to prevent chosen-ciphertext attacks.", ["RSA-OAEP", "padding", "CCA", "public key"], False),
    ("Diffie-Hellman Key Exchange", "Implement ephemeral Diffie-Hellman to establish shared secrets over an insecure channel.", ["Diffie-Hellman", "ephemeral", "shared secret", "discrete log"], False),
    ("ECDH and Curve25519", "Use ECDH on Curve25519 for fast, secure key agreement resistant to side channels.", ["ECDH", "Curve25519", "key agreement", "side channel"], False),
    ("Digital Signatures (EdDSA, ECDSA)", "Sign messages with EdDSA or ECDSA, noting canonical signatures and nonce risks.", ["EdDSA", "ECDSA", "nonce", "canonical"], False),
    ("Certificate Transparency", "Validate X.509 certificates against CT logs to detect mis-issuance.", ["certificate transparency", "CT logs", "X.509", "mis-issuance"], False),
    ("TLS 1.3 Handshake", "Walk through the TLS 1.3 1-RTT handshake with key share and HKDF.", ["TLS 1.3", "1-RTT", "key share", "HKDF"], False),
    ("Perfect Forward Secrecy", "Achieve perfect forward secrecy with ephemeral key exchange so past traffic resists future key compromise.", ["PFS", "ephemeral", "key exchange", "compromise"], False),
    ("mTLS (Mutual TLS)", "Configure mutual TLS so both client and server present and verify certificates.", ["mTLS", "client cert", "verification", "trust"], False),
    ("Zero-Trust Architecture", "Design a zero-trust architecture authenticating every request without network trust.", ["zero trust", "per-request auth", "no implicit trust", "policy"], False),
    ("Hashing vs Encryption vs Encoding", "Distinguish hashing, encryption, and encoding and pick the right tool for each task.", ["hashing", "encryption", "encoding", "purpose"], False),
    ("Merkle Tree for Integrity", "Use Merkle trees to verify integrity of large data sets with compact proofs.", ["Merkle tree", "integrity", "proof", "tamper detection"], False),
    ("Salted Hashing for Storage", "Store credentials as salted slow hashes to resist offline cracking.", ["salted hash", "slow hash", "storage", "cracking"], False),
    ("Rainbow Tables and Defense", "Defend against rainbow tables using salts and memory-hard hashing.", ["rainbow table", "salt", "memory-hard", "precomputed"], False),
    ("PBKDF2 vs bcrypt vs argon2", "Compare PBKDF2, bcrypt, and argon2 on CPU/memory hardness and suitability.", ["PBKDF2", "bcrypt", "argon2", "memory-hard"], False),
    ("Constant-Time Comparison", "Implement constant-time comparison to avoid leaking equality via timing.", ["constant time", "timing leak", "comparison", "secret"], False),
    ("Timing Attacks", "Reason about timing side channels that leak secrets through response time variation.", ["timing attack", "side channel", "leak", "secret"], False),
    ("Side-Channel Attacks", "Defend against cache, power, and EM side channels in sensitive code.", ["side channel", "cache", "power", "EM"], False),
    ("Padding Oracle Attacks", "Prevent padding oracle attacks by using AEAD instead of CBC with MAC-then-encrypt.", ["padding oracle", "CBC", "AEAD", "MAC"], False),
    ("Lucky13 Attack", "Understand the Lucky13 timing attack on TLS CBC and prefer AEAD ciphers.", ["Lucky13", "TLS CBC", "timing", "AEAD"], False),
    ("BEAST/CRIME/BREACH Attacks", "Understand compression and CBC attacks (BEAST, CRIME, BREACH) and their mitigations.", ["BEAST", "CRIME", "BREACH", "compression"], False),
    ("Replay Attacks and Nonces", "Defend against replay using nonces, timestamps, and sequence numbers.", ["replay", "nonce", "timestamp", "sequence"], False),
    ("RBAC vs ABAC", "Choose between role-based and attribute-based access control for authorization.", ["RBAC", "ABAC", "authorization", "policy"], False),
    ("Capability-Based Security", "Design authorization around unforgeable capabilities rather than identity-based ACLs.", ["capabilities", "unforgeable", "delegation", "ACL"], False),
    ("Principle of Least Privilege", "Apply least privilege by granting the minimum scopes needed for a task.", ["least privilege", "scoping", "minimization", "principle"], False),
    ("Sandboxing (seccomp, namespaces)", "Sandbox untrusted code with seccomp filters and Linux namespaces.", ["sandbox", "seccomp", "namespaces", "syscall filter"], False),
    ("Container Security", "Harden containers with reduced capabilities, read-only roots, and minimal images.", ["container", "capabilities", "read-only", "image"], False),
    ("Secret Rotation and KMS", "Rotate secrets via a KMS with envelope encryption and audit logging.", ["secret rotation", "KMS", "envelope encryption", "audit"], False),
    ("Rate Limiting for Security", "Apply rate limits and account lockouts to slow credential-stuffing and brute force.", ["rate limiting", "brute force", "lockout", "account"], False),
    ("Honeypots and Intrusion Detection", "Deploy honeypots and IDS to detect and analyze attackers.", ["honeypot", "IDS", "detection", "deception"], False),
]

CATEGORIES.append(("Security", 461, security))


# ---------------------------------------------------------------------------
# 9. Networking & Protocols  (ids 511-550, 40 questions)
# ---------------------------------------------------------------------------
networking = [
    ("TCP Three-Way Handshake", "Explain SYN, SYN-ACK, ACK and the resulting connection state machine.", ["TCP", "handshake", "SYN", "state machine"], False),
    ("TCP Congestion Control (BBR)", "Explain BBR's model-based congestion control versus loss-based schemes.", ["BBR", "congestion", "bandwidth", "RTT"], False),
    ("TCP Congestion Control (CUBIC)", "Explain the CUBIC cubic window growth function used as default Linux congestion control.", ["CUBIC", "cubic", "window", "congestion"], False),
    ("Slow Start and Congestion Avoidance", "Reason about slow-start, congestion-avoidance, and the ssthresh transition.", ["slow start", "congestion avoidance", "ssthresh", "cwnd"], False),
    ("Fast Retransmit and Fast Recovery", "Recover from packet loss without timing out using duplicate ACKs.", ["fast retransmit", "fast recovery", "dup ACK", "loss"], False),
    ("Nagle's Algorithm", "Explain Nagle's algorithm and its interaction with delayed ACK and latency.", ["Nagle", "delayed ACK", "coalescing", "latency"], False),
    ("TCP Delayed ACK", "Explain delayed ACK and the latency it can introduce with small writes.", ["delayed ACK", "Nagle", "latency", "writes"], False),
    ("TCP Keepalive", "Configure keepalives to detect dead peers without application-level heartbeats.", ["keepalive", "dead peer", "idle", "probes"], False),
    ("TIME_WAIT and 2MSL", "Explain TIME_WAIT, the 2MSL duration, and its role in preventing old segments.", ["TIME_WAIT", "2MSL", "segments", "reuse"], False),
    ("SYN Cookies", "Defend against SYN floods with stateless SYN cookies encoded in the sequence number.", ["SYN cookies", "SYN flood", "stateless", "DoS"], False),
    ("Sliding Window Flow Control", "Implement sliding-window flow control between sender and receiver.", ["sliding window", "flow control", "window", "backpressure"], False),
    ("Head-of-Line Blocking", "Explain TCP head-of-line blocking and how QUIC addresses it.", ["HoL blocking", "TCP", "QUIC", "stream multiplexing"], False),
    ("QUIC Protocol", "Explain QUIC's UDP-based streams, 0-RTT, and transport-level encryption.", ["QUIC", "UDP", "0-RTT", "streams"], False),
    ("HTTP/2 Multiplexing", "Multiplex many streams over a single HTTP/2 connection to avoid connection sprawl.", ["HTTP/2", "multiplexing", "streams", "framing"], False),
    ("HTTP/2 HPACK Header Compression", "Compress HTTP/2 headers with HPACK static and dynamic tables plus Huffman coding.", ["HPACK", "header compression", "Huffman", "dynamic table"], False),
    ("HTTP/3 over QUIC", "Map HTTP/3 semantics onto QUIC streams and frames.", ["HTTP/3", "QUIC", "streams", "frames"], False),
    ("HTTP/1.1 Keep-Alive and Pipelining", "Use keep-alive connections and reason about pipelining's HOL limitations.", ["HTTP/1.1", "keep-alive", "pipelining", "HoL"], False),
    ("HTTP Status Codes and Semantics", "Choose correct HTTP status codes reflecting safe, idempotent, and cacheable semantics.", ["HTTP status", "safe", "idempotent", "cacheable"], False),
    ("HTTP Caching (ETag, Cache-Control)", "Configure conditional and freshness caching with ETag and Cache-Control.", ["caching", "ETag", "Cache-Control", "freshness"], False),
    ("Conditional Requests (If-None-Match)", "Use If-None-Match and If-Modified-Since to validate cached responses.", ["conditional request", "If-None-Match", "If-Modified-Since", "304"], False),
    ("Cookies vs Tokens over HTTP", "Contrast cookie-based and token-based authentication over HTTP and their tradeoffs.", ["cookies", "tokens", "CORS", "CSRF"], False),
    ("TLS Record Layer", "Explain TLS record framing, sequence numbers, and AEAD protection.", ["TLS record", "framing", "sequence", "AEAD"], False),
    ("0-RTT TLS", "Achieve 0-RTT resumption and reason about its replay risk for non-idempotent requests.", ["0-RTT", "resumption", "replay", "TLS 1.3"], False),
    ("DNS Resolution and Caching", "Explain iterative and recursive DNS resolution and the role of resolver caches.", ["DNS", "recursive", "iterative", "cache"], False),
    ("DNS over HTTPS/TLS", "Encrypt DNS queries with DoH/DoT and reason about privacy and policy implications.", ["DoH", "DoT", "encryption", "privacy"], False),
    ("CDN Edge Caching", "Use CDN edge caching with cache keys, purge, and origin shields to reduce latency.", ["CDN", "edge cache", "purge", "origin shield"], False),
    ("Anycast Routing", "Route users to the nearest POP using BGP anycast for low-latency edge services.", ["anycast", "BGP", "POP", "latency"], False),
    ("BGP Basics", "Explain BGP path selection and how AS-path attributes drive interdomain routing.", ["BGP", "AS path", "interdomain", "routing"], False),
    ("NAT and Port Mapping", "Reason about NAT, port translation, and the end-to-end concerns they raise.", ["NAT", "PAT", "port mapping", "end-to-end"], False),
    ("UDP Reliability at Application Layer", "Build reliability, ordering, and congestion control on top of UDP.", ["UDP", "reliability", "ordering", "congestion"], False),
    ("WebRTC and ICE/STUN/TURN", "Establish peer-to-peer media with ICE candidate gathering and TURN fallback.", ["WebRTC", "ICE", "STUN", "TURN"], False),
    ("WebSocket Protocol", "Upgrade to WebSocket for bidirectional, low-latency messaging with framing and ping/pong.", ["WebSocket", "upgrade", "framing", "ping/pong"], False),
    ("Server-Sent Events (SSE)", "Stream server-to-client events over a long-lived HTTP connection with EventSource.", ["SSE", "EventSource", "streaming", "reconnect"], False),
    ("gRPC and Protocol Buffers", "Design gRPC services with protobuf IDL, streaming, and HTTP/2 transport.", ["gRPC", "protobuf", "HTTP/2", "streaming"], False),
    ("Thrift and Avro", "Contrast Thrift and Avro serialization and RPC frameworks.", ["Thrift", "Avro", "serialization", "RPC"], False),
    ("MQTT for IoT", "Use MQTT topics, QoS levels, and retained messages for constrained IoT devices.", ["MQTT", "QoS", "topics", "retained"], False),
    ("AMQP Message Protocol", "Explain AMQP exchanges, queues, and bindings for routed messaging.", ["AMQP", "exchanges", "bindings", "queues"], False),
    ("ICMP and Ping/Traceroute", "Use ICMP echo and time-exceeded messages to implement ping and traceroute.", ["ICMP", "ping", "traceroute", "TTL"], False),
    ("IPv6 Addressing", "Explain IPv6 address structure, subnetting, and stateless autoconfiguration.", ["IPv6", "subnetting", "SLAAC", "addressing"], False),
    ("NAT64 and IPv6 Transition", "Transition between IPv6-only and IPv4 networks using NAT64 and DNS64.", ["NAT64", "DNS64", "transition", "IPv6"], False),
]

CATEGORIES.append(("Networking & Protocols", 511, networking))


# ---------------------------------------------------------------------------
# Assemble the final list with sequential IDs 1..550
# ---------------------------------------------------------------------------
questions = []
qid = 1
for category, start_id, entries in CATEGORIES:
    assert start_id == qid, f"ID mismatch for {category}: expected {qid}, got start {start_id} (count {len(entries)})"
    for title, desc, concepts, implemented in entries:
        questions.append({
            "id": qid,
            "category": category,
            "title": title,
            "description": desc,
            "difficulty": "Hard",
            "concepts": concepts,
            "implemented": implemented,
        })
        qid += 1

assert len(questions) == 550, f"Expected 550 questions, got {len(questions)}"

# Validate the 30 implemented IDs match the spec exactly
SPEC_IMPLEMENTED_IDS = {
    # Concurrency
    1, 2, 3, 11, 14, 16, 21, 30, 32, 34,
    # Data Structures
    82, 84, 85, 89, 90, 96, 97, 101, 102, 104,
    # Algorithms
    162, 163, 165, 168, 177, 181, 183, 189, 195, 215,
}
actual_implemented = {q["id"] for q in questions if q["implemented"]}
assert actual_implemented == SPEC_IMPLEMENTED_IDS, (
    f"Implemented IDs mismatch.\nExpected only: {sorted(SPEC_IMPLEMENTED_IDS)}\n"
    f"Got: {sorted(actual_implemented)}\n"
    f"Missing: {sorted(SPEC_IMPLEMENTED_IDS - actual_implemented)}\n"
    f"Extra: {sorted(actual_implemented - SPEC_IMPLEMENTED_IDS)}"
)

# Verify the implemented questions have the exact titles required
SPEC_TITLES = {
    3: "Lock-Free Treiber Stack",
    2: "Michael-Scott Lock-Free MPMC Queue",
    1: "Lock-Free MPSC Queue",
    32: "SPSC Ring Buffer",
    11: "Ticket Spinlock",
    14: "Readers-Writer Lock",
    16: "Condition Variable",
    21: "Work-Stealing Deque (Chase-Lev)",
    30: "Semaphore",
    34: "Concurrent Hash Map with Lock Striping",
    84: "Skip List",
    101: "LRU Cache",
    102: "LFU Cache",
    97: "Bloom Filter",
    104: "Disjoint Set Union-Find",
    89: "Segment Tree with Lazy Propagation",
    90: "Fenwick Tree / BIT",
    85: "Compressed Trie / Patricia",
    82: "B-Tree",
    96: "Red-Black Tree",
    162: "Edit Distance",
    163: "LIS in O(n log n)",
    177: "Dijkstra with Decrease-Key",
    189: "KMP String Matching",
    195: "Convex Hull",
    165: "Knapsack 0/1",
    183: "Max Flow: Dinic's",
    181: "Floyd-Warshall APSP",
    215: "Quickselect",
    168: "Manacher's Longest Palindrome",
}
id_to_q = {q["id"]: q for q in questions}
for sid, expected_title in SPEC_TITLES.items():
    assert sid in id_to_q, f"Spec ID {sid} not found"
    actual = id_to_q[sid]["title"]
    assert actual == expected_title, f"Title mismatch id={sid}: expected '{expected_title}', got '{actual}'"

# Verify every question has a valid category
VALID_CATEGORIES = {
    "Concurrency", "Data Structures", "Algorithms", "SQL & Database Design",
    "System Design", "Memory Management", "Performance & Profiling",
    "Security", "Networking & Protocols",
}
for q in questions:
    assert q["category"] in VALID_CATEGORIES, f"Bad category for id {q['id']}: {q['category']}"
    assert q["difficulty"] == "Hard"
    assert isinstance(q["concepts"], list) and len(q["concepts"]) >= 3
    assert 1 <= len(q["description"].split(".")) <= 4  # roughly 1-2 sentences
    assert isinstance(q["implemented"], bool)

# Count per category
from collections import Counter
counts = Counter(q["category"] for q in questions)
for cat, cnt in counts.most_common():
    print(f"{cat}: {cnt}")

print(f"\nTotal questions: {len(questions)}")
print(f"Implemented: {sum(1 for q in questions if q['implemented'])}")

OUT = "/home/z/my-project/interview-prep/questions.json"
with open(OUT, "w", encoding="utf-8") as f:
    json.dump(questions, f, indent=2, ensure_ascii=False)
    f.write("\n")

print(f"\nWrote {OUT}")
