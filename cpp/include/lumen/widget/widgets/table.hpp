#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct TableColumn { std::string id, label; float width; TableColumn(std::string i, std::string l, float w) : id(std::move(i)), label(std::move(l)), width(w) {} };
struct TableRow { std::vector<std::string> cells; TableRow(std::vector<std::string> c) : cells(std::move(c)) {} };
class Table : public Widget {
public:
    Table(std::vector<TableColumn> columns, std::vector<TableRow> rows) : columns_(std::move(columns)), rows_(std::move(rows)) { style_ = Style().flex_col().bg_white().rounded_md().border(1).border_color_(Color(226,232,240)).text_sm().build(); }
    size_t row_count() const { return rows_.size(); }
    const std::vector<size_t>& selected() const { return selected_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Table"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.fill_rect(Rect::from_xywh(rect.min.x, rect.min.y, rect.width(), 36), Color(248,250,252)); }
private:
    ResolvedStyle style_; std::vector<TableColumn> columns_; std::vector<TableRow> rows_; std::vector<size_t> selected_;
};
} // namespace lumen::widgets
