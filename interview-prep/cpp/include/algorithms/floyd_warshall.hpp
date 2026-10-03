// Question #181: Floyd-Warshall for All-Pairs Shortest Paths
// Category: Algorithms | Difficulty: Hard | Concepts: k-iteration, negative-cycle detection
#pragma once
#include <vector>
#include <cstdint>

namespace interview_prep {

constexpr int64_t FW_INF = (1LL << 60);

inline std::vector<std::vector<int64_t>> floyd_warshall(std::vector<std::vector<int64_t>> dist) {
    int n = dist.size();
    for (int k = 0; k < n; ++k)
        for (int i = 0; i < n; ++i)
            for (int j = 0; j < n; ++j)
                if (dist[i][k] != FW_INF && dist[k][j] != FW_INF)
                    if (dist[i][k] + dist[k][j] < dist[i][j])
                        dist[i][j] = dist[i][k] + dist[k][j];
    return dist;
}

inline bool has_negative_cycle(const std::vector<std::vector<int64_t>>& dist) {
    auto result = floyd_warshall(dist);
    for (int i = 0; i < (int)dist.size(); ++i)
        if (result[i][i] < 0) return true;
    return false;
}

} // namespace interview_prep
