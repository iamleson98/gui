#pragma once
#include "lumen/core/id.hpp"
#include "lumen/core/rect.hpp"
#include "lumen/style/style.hpp"
#include "lumen/event/event.hpp"
#include "lumen/render/painter.hpp"
#include "lumen/layout/layout.hpp"
#include <memory>
#include <vector>
#include <string>
namespace lumen {

class Widget;
struct Element { std::shared_ptr<Widget> widget; Id id; Element()=default; Element(Id i, std::shared_ptr<Widget> w) : widget(std::move(w)), id(i) {} };
struct PaintCtx { Painter& painter; const Theme& theme; const LayoutRect& layout; float scale=1.0f; };

class Widget {
public:
    virtual ~Widget() = default;
    virtual const ResolvedStyle& style() const = 0;
    virtual void paint(PaintCtx& ctx, const Rect& rect) const {}
    virtual EventResult on_event(EventCtx& ctx, const Event& event) { return EventResult::Ignored; }
    virtual const std::vector<Element>& children() const { return empty_; }
    virtual std::vector<Element>& children_mut() { return empty_mut_; }
    virtual std::string debug_name() const { return "Widget"; }
private:
    static inline std::vector<Element> empty_, empty_mut_;
};

class Ui {
public:
    explicit Ui(Id root_id) : id_(root_id) {}
    template<typename W, typename... Args> W& push(Args&&... args) {
        Id child_id = id_.derive_index(next_index_++);
        auto w = std::make_shared<W>(std::forward<Args>(args)...);
        W* ptr = w.get();
        children_.push_back(Element(child_id, std::move(w)));
        return *ptr;
    }
    std::vector<Element> take_children() { return std::move(children_); }
private:
    Id id_; size_t next_index_=0; std::vector<Element> children_;
};
} // namespace lumen
