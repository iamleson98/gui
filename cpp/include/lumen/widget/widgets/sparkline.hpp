#pragma once
#include "lumen/widget/widget.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include <string>
#include <functional>
#include <cmath>
#include <algorithm>

namespace lumen::widgets {

class Sparkline : public Widget {
public:
    Sparkline() { style_ = Style().w_full().h(32).build(); }
    void push(float v) { data_.push_back(v); recompute(); }
    void clear() { data_.clear(); range_min_=0; range_max_=1; }
    const ResolvedStyle& style() const override { return style_; }
    std::string debug_name() const override { return "Sparkline"; }
    void paint(PaintCtx& ctx, const Rect& rect) const override {
        size_t n=data_.size(); if(n==0) return;
        auto pt=[&](size_t i)->Vec2{ float tx=n>1?i/(float)(n-1):0.5f; float ty=std::clamp((data_[i]-range_min_)/std::max(1e-6f,range_max_-range_min_),0.0f,1.0f); return Vec2(rect.min.x+rect.width()*tx, rect.min.y+rect.height()*(1-ty)); };
        if(n==1) { auto p=pt(0); ctx.painter.fill_rect(Rect::from_xywh(p.x-2,p.y-2,4,4), Color::TW_INDIGO_500); return; }
        auto prev=pt(0); for(size_t i=1; i<n; ++i) { auto p=pt(i); ctx.painter.fill_rect(Rect::from_corners(prev,p), Color::TW_INDIGO_500); prev=p; }
    }
private:
    void recompute() { if(data_.empty()) return; float mn=1e30f,mx=-1e30f; for(float v:data_){mn=std::min(mn,v);mx=std::max(mx,v);} if(mx-mn<1e-6f){mx+=1;mn-=1;} range_min_=mn; range_max_=mx; }
    ResolvedStyle style_; std::vector<float> data_; float range_min_=0, range_max_=1;
};
} // namespace lumen::widgets
