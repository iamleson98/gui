#include "lumen/platform/app.hpp"
#include "lumen/layout/layout.hpp"
#include "lumen/render/painter.hpp"
#include "lumen/render/gl_backend.hpp"
#include "lumen/render/software.hpp"
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

    // Build the view tree once (used by both GPU and software paths).
    Ui ui(Id::from_str("root"));
    app.view(ui);
    auto tree = ui.take_children();

    Vec2 vp(builder.width, builder.height);
    auto node = build_node(tree);
    auto layout = arrange(node, vp);

    // Paint into a CPU mesh (same for both paths).
    Painter painter;
    paint_tree(tree, layout, builder.theme, painter);
    const Mesh& mesh = painter.mesh();

    // Try the GPU window first. If it fails (no X server, no GLX, macOS,
    // Windows, etc.), fall back to the software renderer which outputs a
    // PNG file.
    GLWindow win;
    GLWindowConfig cfg;
    cfg.title = builder.title;
    cfg.width = builder.width;
    cfg.height = builder.height;
    cfg.vsync = builder.vsync;

    if (win.init(cfg)) {
        // GPU path: re-paint every frame inside the event loop.
        win.run([&](Painter& frame_painter) {
            Ui frame_ui(Id::from_str("root"));
            app.view(frame_ui);
            auto frame_tree = frame_ui.take_children();
            Vec2 frame_vp(win.width(), win.height());
            auto frame_node = build_node(frame_tree);
            auto frame_layout = arrange(frame_node, frame_vp);
            frame_painter.clear();
            paint_tree(frame_tree, frame_layout, builder.theme, frame_painter);
        });
        return 0;
    }

    // Software fallback: rasterize the mesh to a PNG file.
    std::cerr << "lumen: GPU window unavailable, using software renderer.\n";
    SoftwareRendererConfig sw_cfg;
    sw_cfg.width = builder.width;
    sw_cfg.height = builder.height;
    sw_cfg.output_path = "lumen_output.png";
    SoftwareRenderer sw;
    sw.init(sw_cfg);
    sw.render(mesh, builder.theme.palette.bg);

    std::cout << "lumen: rendered " << mesh.vertex_count() << " vertices ("
              << mesh.index_count() / 3 << " triangles) to "
              << sw_cfg.output_path << " (" << sw_cfg.width << "x"
              << sw_cfg.height << ")\n";
    std::cout << "lumen: open " << sw_cfg.output_path
              << " to view the rendered UI.\n";
    return 0;
}
} // namespace lumen
