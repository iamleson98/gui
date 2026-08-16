use crate::core::{Id, Rect, ScaleFactor, Vec2};
use crate::event::{Event as LumenEvent, EventState, MessageBus};
use crate::layout::{arrange, LayoutNode, LayoutRect};
use crate::render::{Painter, WgpuRenderer};
use crate::style::{Style, Theme, Tw};
use crate::text::TextEngine;
use crate::widget::{Element, PaintCtx, Ui};
use std::collections::HashMap;

pub trait App: 'static + Sized {
    type State;
    type Message: 'static + Send + Clone;
    fn init() -> Self::State;
    fn view(state: &Self::State, ui: &mut Ui);
    fn update(state: &mut Self::State, msg: Self::Message);
}

pub struct AppBuilder {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub theme: Theme,
    pub vsync: bool,
}
impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            title: "lumen app".into(),
            width: 1280,
            height: 720,
            theme: Theme::light(),
            vsync: true,
        }
    }
}
impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into();
        self
    }
    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.width = w;
        self.height = h;
        self
    }
    pub fn theme(mut self, t: Theme) -> Self {
        self.theme = t;
        self
    }
}

struct AppState<A: App> {
    builder: AppBuilder,
    state: A::State,
    theme: Theme,
    renderer: Option<WgpuRenderer>,
    text: TextEngine,
    message_bus: MessageBus,
    last_tree: Vec<Element>,
    last_layout: Option<LayoutRect>,
    needs_redraw: bool,
    cursor_pos: Vec2,
    focus_state: HashMap<Id, bool>,
}
impl<A: App> AppState<A> {
    fn new(builder: AppBuilder) -> Self {
        let theme = builder.theme.clone();
        let state = A::init();
        Self {
            builder,
            state,
            theme,
            renderer: None,
            text: TextEngine::new(),
            message_bus: MessageBus::default(),
            last_tree: Vec::new(),
            last_layout: None,
            needs_redraw: true,
            cursor_pos: Vec2::ZERO,
            focus_state: HashMap::new(),
        }
    }
}

impl<A: App> winit::application::ApplicationHandler for AppState<A> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attrs = winit::window::Window::default_attributes()
            .with_title(&self.builder.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.builder.width,
                self.builder.height,
            ));
        let win = match event_loop.create_window(attrs) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("lumen: failed to create window: {e}");
                event_loop.exit();
                return;
            }
        };
        let size = win.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });
        let surface = match instance.create_surface(win) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("lumen: failed to create surface: {e}");
                event_loop.exit();
                return;
            }
        };
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })) {
                Some(a) => a,
                None => {
                    eprintln!("lumen: no GPU adapter");
                    event_loop.exit();
                    return;
                }
            };
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let config = crate::render::SurfaceConfig {
            width: size.width,
            height: size.height,
            format,
            present_mode: if self.builder.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            alpha_mode: caps.alpha_modes[0],
        };
        self.renderer = Some(
            match pollster::block_on(WgpuRenderer::new_with_adapter(surface, config, adapter)) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("lumen: renderer init failed: {e}");
                    event_loop.exit();
                    return;
                }
            },
        );
        self.needs_redraw = true;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _wid: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        use winit::event::WindowEvent;
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(r) = self.renderer.as_mut() {
                    r.resize(size.width, size.height);
                }
                self.needs_redraw = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = Vec2::new(position.x as f32, position.y as f32);
                dispatch_event::<A>(
                    &mut self.last_tree,
                    &mut self.last_layout,
                    &self.focus_state,
                    LumenEvent::PointerMove {
                        pos: self.cursor_pos,
                    },
                    &mut self.state,
                    &mut self.message_bus,
                );
                self.needs_redraw = true;
            }
            WindowEvent::CursorLeft { .. } => {
                dispatch_event::<A>(
                    &mut self.last_tree,
                    &mut self.last_layout,
                    &self.focus_state,
                    LumenEvent::PointerLeave,
                    &mut self.state,
                    &mut self.message_bus,
                );
                self.needs_redraw = true;
            }
            WindowEvent::MouseInput {
                state: ms, button, ..
            } => {
                let btn = match button {
                    winit::event::MouseButton::Left => crate::input::MouseButton::Left,
                    winit::event::MouseButton::Right => crate::input::MouseButton::Right,
                    _ => crate::input::MouseButton::Other(0),
                };
                let ev = match ms {
                    winit::event::ElementState::Pressed => LumenEvent::PointerDown {
                        pos: self.cursor_pos,
                        button: btn,
                    },
                    winit::event::ElementState::Released => LumenEvent::PointerUp {
                        pos: self.cursor_pos,
                        button: btn,
                    },
                };
                dispatch_event::<A>(
                    &mut self.last_tree,
                    &mut self.last_layout,
                    &self.focus_state,
                    ev,
                    &mut self.state,
                    &mut self.message_bus,
                );
                self.needs_redraw = true;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                use winit::keyboard::{KeyCode as WKC, PhysicalKey};
                let code = match event.physical_key {
                    PhysicalKey::Code(WKC::Escape) => Some(crate::input::KeyCode::Escape),
                    PhysicalKey::Code(WKC::Enter) => Some(crate::input::KeyCode::Enter),
                    PhysicalKey::Code(WKC::Space) => Some(crate::input::KeyCode::Space),
                    PhysicalKey::Code(WKC::Backspace) => Some(crate::input::KeyCode::Backspace),
                    PhysicalKey::Code(WKC::ArrowLeft) => Some(crate::input::KeyCode::ArrowLeft),
                    PhysicalKey::Code(WKC::ArrowRight) => Some(crate::input::KeyCode::ArrowRight),
                    PhysicalKey::Code(WKC::ArrowUp) => Some(crate::input::KeyCode::ArrowUp),
                    PhysicalKey::Code(WKC::ArrowDown) => Some(crate::input::KeyCode::ArrowDown),
                    _ => None,
                };
                if let Some(code) = code {
                    let ev = match event.state {
                        winit::event::ElementState::Pressed => LumenEvent::KeyDown {
                            code,
                            modifiers: crate::input::Modifiers::empty(),
                        },
                        _ => LumenEvent::KeyUp {
                            code,
                            modifiers: crate::input::Modifiers::empty(),
                        },
                    };
                    dispatch_event::<A>(
                        &mut self.last_tree,
                        &mut self.last_layout,
                        &self.focus_state,
                        ev,
                        &mut self.state,
                        &mut self.message_bus,
                    );
                    self.needs_redraw = true;
                }
            }
            WindowEvent::RedrawRequested => {
                self.needs_redraw = true;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _el: &winit::event_loop::ActiveEventLoop) {
        if self.needs_redraw {
            if let Some(r) = self.renderer.as_mut() {
                if let Err(e) = redraw::<A>(
                    &mut self.state,
                    &self.theme,
                    &mut self.last_tree,
                    &mut self.last_layout,
                    r,
                    &mut self.text,
                ) {
                    if !matches!(e, wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) {
                        eprintln!("lumen: render error: {e}");
                    }
                }
            }
            self.needs_redraw = false;
        }
    }
}

pub fn run<A: App>(builder: AppBuilder) {
    let event_loop = winit::event_loop::EventLoop::new().expect("failed to create event loop");
    let mut app_state = AppState::<A>::new(builder);
    event_loop
        .run_app(&mut app_state)
        .expect("event loop failed");
}

fn dispatch_event<A: App>(
    tree: &mut Vec<Element>,
    layout: &mut Option<LayoutRect>,
    _focus: &HashMap<Id, bool>,
    event: LumenEvent,
    state: &mut A::State,
    bus: &mut MessageBus,
) {
    let mut es = EventState::default();
    if let Some(layout) = layout.as_ref() {
        let pos = event.position();
        if let Some(pos) = pos {
            if let Some(tid) = hit_test(layout, pos) {
                let rect = find_rect(layout, tid).unwrap_or(Rect::ZERO);
                dispatch_to_tree(tree, tid, rect, &event, &mut es);
            }
        } else {
            dispatch_to_all(tree, &event, &mut es);
        }
    }
    for m in es.messages.drain(..) {
        bus.dispatch(m.as_ref());
    }
    let _ = state;
}

fn hit_test(layout: &LayoutRect, pos: Vec2) -> Option<Id> {
    for c in layout.children.iter().rev() {
        if let Some(id) = hit_test(c, pos) {
            return Some(id);
        }
    }
    if layout.rect.contains(pos) {
        Some(layout.id)
    } else {
        None
    }
}
fn find_rect(layout: &LayoutRect, id: Id) -> Option<Rect> {
    if layout.id == id {
        return Some(layout.rect);
    }
    for c in &layout.children {
        if let Some(r) = find_rect(c, id) {
            return Some(r);
        }
    }
    None
}
fn dispatch_to_tree(
    tree: &mut [Element],
    tid: Id,
    rect: Rect,
    event: &LumenEvent,
    state: &mut EventState,
) {
    for el in tree.iter_mut() {
        if el.id == tid {
            let mut ctx = crate::event::EventCtx {
                current_id: el.id,
                current_rect: rect,
                state,
            };
            el.inner.on_event(&mut ctx, event);
            return;
        }
        dispatch_to_tree(el.inner.children_mut(), tid, rect, event, state);
    }
}
fn dispatch_to_all(tree: &mut [Element], event: &LumenEvent, state: &mut EventState) {
    for el in tree.iter_mut() {
        let mut ctx = crate::event::EventCtx {
            current_id: el.id,
            current_rect: Rect::ZERO,
            state,
        };
        el.inner.on_event(&mut ctx, event);
        dispatch_to_all(el.inner.children_mut(), event, state);
    }
}

fn redraw<A: App>(
    state: &mut A::State,
    theme: &Theme,
    last_tree: &mut Vec<Element>,
    last_layout: &mut Option<LayoutRect>,
    renderer: &mut WgpuRenderer,
    text: &mut TextEngine,
) -> Result<(), wgpu::SurfaceError> {
    let mut ui = Ui::new(Id::new("root"));
    A::view(state, &mut ui);
    *last_tree = ui.into_children();
    let viewport = Vec2::new(renderer.config.width as f32, renderer.config.height as f32);
    let layout_node = build_layout_node_recursive(last_tree, text);
    let layout = arrange(&layout_node, viewport);
    *last_layout = Some(layout.clone());
    let mut painter = Painter::new();
    paint_tree_recursive(last_tree, &layout, theme, &mut painter, text);

    // Upload glyph atlas to GPU if it changed
    if text.atlas_dirty {
        let (aw, ah) = text.atlas_size();
        renderer.upload_glyph_atlas(text.atlas_pixels(), aw, ah);
        text.atlas_dirty = false;
    }

    renderer.render(painter.mesh(), theme.palette.bg)?;
    Ok(())
}

fn build_layout_node_recursive<'a>(tree: &'a [Element], text: &mut TextEngine) -> LayoutNode<'a> {
    let root_style = Box::leak(Box::new(
        Style::new()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .w_full()
            .h_full()
            .build(),
    ));
    let children: Vec<LayoutNode<'a>> = tree
        .iter()
        .map(|el| build_layout_node_for_element(el, text))
        .collect();
    LayoutNode {
        id: Id::new("root"),
        style: root_style,
        measure: None,
        children,
        intrinsic_size: None,
    }
}
fn build_layout_node_for_element<'a>(el: &'a Element, text: &mut TextEngine) -> LayoutNode<'a> {
    // Measure this widget's own text first (if any). Must happen before the
    // recursive call so the mutable borrow of `text` ends before we recurse.
    let intrinsic_size = el
        .inner
        .text_measure()
        .map(|(t, fs)| text.measure_text(t, fs));

    let children: Vec<LayoutNode<'a>> = el
        .inner
        .children()
        .iter()
        .map(|c| build_layout_node_for_element(c, text))
        .collect();
    LayoutNode {
        id: el.id,
        style: el.inner.style(),
        measure: None,
        children,
        intrinsic_size,
    }
}
fn paint_tree_recursive(
    tree: &[Element],
    layout: &LayoutRect,
    theme: &Theme,
    painter: &mut Painter,
    text: &mut TextEngine,
) {
    for (el, child_layout) in tree.iter().zip(layout.children.iter()) {
        let mut ctx = PaintCtx {
            painter,
            theme,
            layout: child_layout,
            scale: ScaleFactor::IDENT,
            text,
        };
        el.inner.paint(&mut ctx, &child_layout.rect);
        paint_tree_recursive(
            el.inner.children(),
            child_layout,
            theme,
            ctx.painter,
            ctx.text,
        );
    }
}
