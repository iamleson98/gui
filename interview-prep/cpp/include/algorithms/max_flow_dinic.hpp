// Question #183: Max Flow: Dinic's Algorithm
// Category: Algorithms | Difficulty: Hard | Concepts: Dinic, level graph, blocking flow
#pragma once
#include <vector>
#include <queue>
#include <cstdint>

namespace interview_prep {

class MaxFlow {
    struct Edge { int to; int64_t cap; int rev; };
    int n_;
    std::vector<std::vector<Edge>> graph_;

    bool bfs(int s, int t, std::vector<int>& level) {
        std::fill(level.begin(), level.end(), -1);
        level[s] = 0;
        std::queue<int> q;
        q.push(s);
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (auto& e : graph_[u]) {
                if (e.cap > 0 && level[e.to] < 0) {
                    level[e.to] = level[u] + 1;
                    q.push(e.to);
                }
            }
        }
        return level[t] >= 0;
    }

    int64_t dfs(int u, int t, int64_t f, std::vector<int>& level, std::vector<int>& iter) {
        if (u == t) return f;
        for (; iter[u] < (int)graph_[u].size(); ++iter[u]) {
            Edge& e = graph_[u][iter[u]];
            if (e.cap > 0 && level[e.to] == level[u] + 1) {
                int64_t d = dfs(e.to, t, std::min(f, e.cap), level, iter);
                if (d > 0) {
                    e.cap -= d;
                    graph_[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        return 0;
    }

public:
    explicit MaxFlow(int n) : n_(n), graph_(n) {}

    void add_edge(int from, int to, int64_t cap) {
        graph_[from].push_back({to, cap, (int)graph_[to].size()});
        graph_[to].push_back({from, 0, (int)graph_[from].size() - 1});
    }

    int64_t max_flow(int s, int t) {
        int64_t flow = 0;
        std::vector<int> level(n_), iter(n_);
        while (bfs(s, t, level)) {
            std::fill(iter.begin(), iter.end(), 0);
            while (true) {
                int64_t f = dfs(s, t, (1LL << 60), level, iter);
                if (f == 0) break;
                flow += f;
            }
        }
        return flow;
    }
};

} // namespace interview_prep
