// Question #195: Convex Hull (Andrew's Monotone Chain)
// Category: Algorithms | Difficulty: Hard | Concepts: monotone chain, cross product
#pragma once
#include <vector>
#include <algorithm>

namespace interview_prep {

struct Point { double x, y; };

inline double cross(Point o, Point a, Point b) {
    return (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
}

inline std::vector<Point> convex_hull(std::vector<Point> points) {
    int n = points.size();
    if (n <= 2) return points;
    std::sort(points.begin(), points.end(), [](const Point& a, const Point& b) {
        if (a.x != b.x) return a.x < b.x;
        return a.y < b.y;
    });
    std::vector<Point> lower;
    for (auto& p : points) {
        while (lower.size() >= 2 && cross(lower[lower.size()-2], lower[lower.size()-1], p) <= 0)
            lower.pop_back();
        lower.push_back(p);
    }
    std::vector<Point> upper;
    for (int i = n - 1; i >= 0; --i) {
        auto& p = points[i];
        while (upper.size() >= 2 && cross(upper[upper.size()-2], upper[upper.size()-1], p) <= 0)
            upper.pop_back();
        upper.push_back(p);
    }
    lower.pop_back();
    upper.pop_back();
    lower.insert(lower.end(), upper.begin(), upper.end());
    return lower;
}

} // namespace interview_prep
