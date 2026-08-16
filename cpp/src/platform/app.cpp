#include "lumen/platform/app.hpp"
#include "lumen/layout/layout.hpp"
#include "lumen/render/painter.hpp"
#include "lumen/render/gl_backend.hpp"
#include <iostream>

namespace lumen {

// Root layout: top-aligned, full-width, 32px padding, 16px gap between items.
// This matches the Rust platform/winit_app.rs root layout.
static LayoutNode build_node(const std::vector<Element>& tree) {
    static auto rs = Style()
        .flex()
        .flex_col()
        .items_center()
        .justify_start()
        .gap_4()
        .p_8()
        .w_full()
        .h_full()
        .build();
    LayoutNode node; node.id = Id::from_str("root"); node.style = &rs;
    for(auto& el : tree) {
        LayoutNode c;
        c.id = el.id;
        c.style = &el.widget->style();
        auto sub = build_node(el.widget->children());
        c.children = std::move(sub.children);
        node.children.push_back(std::move(c));
    }
    return node;
}

static void paint_tree(const std::vector<Element>& tree, const LayoutRect& layout, const Theme& theme, Painter& painter) {
    for(size_t i=0; i<tree.size() && i<layout.children.size(); ++i) {
        PaintCtx ctx{painter, theme, layout.children[i], 1.0f};
        tree[i].widget->paint(ctx, layout.children[i].rect);
        paint_tree(tree[i].widget->children(), layout.children[i], theme, painter);
    }
}

int run(App& app, const AppBuilder& builder) {
    app.init();
    GLWindow win;
    GLWindowConfig cfg; cfg.title=builder.title; cfg.width=builder.width; cfg.height=builder.height; cfg.vsync=builder.vsync;
    if(!win.init(cfg)) {
        std::cerr << "lumen: falling back to headless mode.\n"
                  << "lumen: the app will render one frame to the CPU mesh and exit.\n"
                  << "lumen: to get a real GPU window, run inside an X session or use:\n"
                  << "lumen:   Xvfb :99 -screen 0 1024x768x24 +extension GLX & DISPLAY=:99 ./counter\n";
        Ui ui(Id::from_str("root")); app.view(ui); auto tree=ui.take_children();
        Vec2 vp(builder.width, builder.height); auto node=build_node(tree); auto layout=arrange(node, vp);
        Painter p; paint_tree(tree, layout, builder.theme, p);
        std::cout << "lumen: headless - " << p.mesh().vertex_count() << " vertices, "
                  << p.mesh().index_count() << " indices\n";
        return 1;
    }
    win.run([&](Painter& painter) {
        Ui ui(Id::from_str("root")); app.view(ui); auto tree=ui.take_children();
        Vec2 vp(win.width(), win.height()); auto node=build_node(tree); auto layout=arrange(node, vp);
        painter.clear(); paint_tree(tree, layout, builder.theme, painter);
    });
    return 0;
}
} // namespace lumen
