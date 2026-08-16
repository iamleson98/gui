#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

struct Date { int year, month, day; Date(int y, int m, int d) : year(y), month(m), day(d) {} static int days_in_month(int y, int m) { switch(m) { case 1:case 3:case 5:case 7:case 8:case 10:case 12: return 31; case 4:case 6:case 9:case 11: return 30; case 2: return is_leap(y)?29:28; } return 30; } static bool is_leap(int y) { return (y%4==0&&y%100!=0)||(y%400==0); } };
class DatePicker : public Widget {
public:
    DatePicker(Date date) : date_(date) { style_ = Style().px_3().py_2().rounded_md().bg_white().border(1).cursor_pointer().text_sm().build(); }
    const Date& date() const { return date_; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "DatePicker"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override { ctx.painter.fill_rounded_rect(rect, style_.background, style_.border_radius); ctx.painter.stroke_rect(rect, style_.border_color, 1.0f); }
private:
    ResolvedStyle style_; Date date_; bool open_=false;
};
} // namespace lumen::widgets
