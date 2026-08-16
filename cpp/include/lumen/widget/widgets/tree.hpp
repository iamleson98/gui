#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct TreeNode { std::string label; std::vector<TreeNode> children; bool expanded=false, disabled=false; TreeNode(std::string l) : label(std::move(l)) {} TreeNode& with_children(std::vector<TreeNode> c) { children=std::move(c); return *this; } TreeNode& expanded_() { expanded=true; return *this; } };
class Tree : public Widget {
public:
    Tree(TreeNode root) : root_(std::move(root)) { style_ = Style().flex_col().bg_white().text_sm().build(); }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Tree"; }
private:
    ResolvedStyle style_; TreeNode root_;
};
} // namespace lumen::widgets
