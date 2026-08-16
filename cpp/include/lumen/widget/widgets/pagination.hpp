#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Pagination : public Widget {
public:
    Pagination(size_t total, size_t current) : total_(total), current_(std::min(current, total>0?total-1:0)) { style_ = Style().flex().gap_1().build(); }
    size_t current() const { return current_; }
    size_t total_pages() const { return total_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Pagination"; }
private:
    ResolvedStyle style_; size_t total_, current_;
};
} // namespace lumen::widgets
