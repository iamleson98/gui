#pragma once
#include "lumen/core/vec2.hpp"
#include "lumen/core/id.hpp"
#include "lumen/core/rect.hpp"
#include "lumen/input/input.hpp"
#include "lumen/style/style.hpp"
#include <vector>
#include <functional>
#include <any>
#include <unordered_map>
#include <typeindex>
#include <memory>
#include <optional>
namespace lumen {

enum class EventResult { Ignored, Handled, HandledAndRedraw };
inline bool is_handled(EventResult r) { return r != EventResult::Ignored; }

struct Event {
    enum class Kind { PointerMove, PointerDown, PointerUp, PointerLeave, Scroll, KeyDown, KeyUp, Char, FocusGained, FocusLost, Resized } kind;
    Vec2 pos, delta; MouseButton button=MouseButton::Left; KeyCode key=KeyCode::Escape; Modifiers modifiers=Modifiers::None; char ch=0; Vec2 size;
    std::optional<Vec2> position() const {
        switch(kind) { case Kind::PointerMove: case Kind::PointerDown: case Kind::PointerUp: case Kind::Scroll: return pos; default: return std::nullopt; }
    }
};

class EventState {
public:
    bool needs_redraw=false, needs_layout=false;
    std::optional<Id> focus_request, pointer_capture;
    std::optional<Cursor> cursor;
    void request_redraw() { needs_redraw=true; }
    void request_layout() { needs_layout=true; needs_redraw=true; }
    void request_focus(Id id) { focus_request=id; }
    void set_cursor(Cursor c) { cursor=c; }
    void capture_pointer(Id id) { pointer_capture=id; }
    void release_pointer() { pointer_capture.reset(); }
    template<typename T> void emit(T v) { messages_.push_back(std::make_any<T>(std::move(v))); }
    template<typename T> size_t message_count() const { size_t n=0; for(auto&m:messages_) if(m.type()==typeid(T)) ++n; return n; }
    template<typename T> std::optional<T> first_message() const { for(auto&m:messages_) if(m.type()==typeid(T)) return std::any_cast<const T&>(m); return std::nullopt; }
    void clear_messages() { messages_.clear(); }
    size_t message_count_total() const { return messages_.size(); }
    const std::vector<std::any>& messages() const { return messages_; }
private:
    std::vector<std::any> messages_;
};

class EventCtx {
public:
    Id current_id; Rect current_rect; EventState& state;
    EventCtx(Id id, Rect r, EventState& s) : current_id(id), current_rect(r), state(s) {}
};

class MessageBus {
public:
    template<typename T> void subscribe(std::function<void(const T&)> f) {
        slots_[std::type_index(typeid(T))].push_back([f=std::move(f)](const std::any& m){ if(m.type()==typeid(T)) f(std::any_cast<const T&>(m)); });
    }
    void dispatch(const std::any& msg) { auto it=slots_.find(std::type_index(msg.type())); if(it!=slots_.end()) for(auto&s:it->second) s(msg); }
private:
    std::unordered_map<std::type_index, std::vector<std::function<void(const std::any&)>>> slots_;
};
} // namespace lumen
