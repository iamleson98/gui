#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct Crumb { std::string label; bool clickable=true; };
class Breadcrumb : public Widget {
public:
    Breadcrumb(std::vector<Crumb> crumbs) : crumbs_(std::move(crumbs)) { style_ = Style().flex().gap_1().items_center().text_sm().build(); }
    const std::vector<Crumb>& crumbs() const { return crumbs_; }
    void push(Crumb c) { crumbs_.push_back(std::move(c)); }
    void pop() { crumbs_.pop_back(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Breadcrumb"; }
private:
    ResolvedStyle style_; std::vector<Crumb> crumbs_;
};
} // namespace lumen::widgets
