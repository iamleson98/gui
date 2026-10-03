// Question #177: Dijkstra's Algorithm with Decrease-Key Heap
// Category: Algorithms | Difficulty: Hard | Concepts: Dijkstra, indexed PQ, decrease-key
#pragma once
#include <vector>
#include <queue>
#include <cstdint>
#include <limits>

namespace interview_prep {

class Graph {
    int n_;
    std::vector<std::vector<std::pair<int, int64_t>>> adj_;

public:
    explicit Graph(int n) : n_(n), adj_(n) {}

    void add_edge(int from, int to, int64_t cost) {
        adj_[from].push_back({to, cost});
    }

    std::vector<int64_t> dijkstra(int source) {
        const int64_t INF = std::numeric_limits<int64_t>::max();
        std::vector<int64_t> dist(n_, INF);
        std::vector<bool> visited(n_, false);
        dist[source] = 0;
        using P = std::pair<int64_t, int>;
        std::priority_queue<P, std::vector<P>, std::greater<P>> pq;
        pq.push({0, source});
        while (!pq.empty()) {
            auto [d, u] = pq.top();
            pq.pop();
            if (visited[u]) continue;
            visited[u] = true;
            for (auto& [v, w] : adj_[u]) {
                if (dist[u] + w < dist[v]) {
                    dist[v] = dist[u] + w;
                    pq.push({dist[v], v});
                }
            }
        }
        return dist;
    }
};

} // namespace interview_prep
