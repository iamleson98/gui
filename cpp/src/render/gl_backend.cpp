#include "lumen/render/gl_backend.hpp"
#include "lumen/render/painter.hpp"
#include <iostream>
#include <cstring>
#include <dlfcn.h>

#if defined(__linux__) || defined(__unix__)
#include <X11/Xlib.h>
#include <X11/Xutil.h>

namespace lumen {

// Bring X11 types into scope (lumen::Display conflicts with ::Display)
using ::Display; using ::Window; using ::Atom; using ::XEvent;
using ::XVisualInfo; using ::XSetWindowAttributes;

// GL types
typedef unsigned int GLenum, GLuint;
typedef int GLint, GLsizei;
typedef float GLfloat;
typedef char GLchar;
typedef unsigned char GLboolean;
typedef void GLvoid;
typedef ptrdiff_t GLsizeiptr;
typedef unsigned int GLbitfield;

// GL constants (prefixed to avoid X11 conflicts)
static const GLenum LT_FLOAT           = 0x1406;
static const GLenum LT_UNSIGNED_SHORT  = 0x1403;
static const GLenum LT_UNSIGNED_INT    = 0x1405;
static const GLenum LT_TRIANGLES       = 0x0004;
static const GLenum LT_ARRAY_BUFFER    = 0x8892;
static const GLenum LT_ELEMENT_ARRAY   = 0x8893;
static const GLenum LT_DYNAMIC_DRAW    = 0x88E8;
static const GLenum LT_VERTEX_SHADER   = 0x8B31;
static const GLenum LT_FRAGMENT_SHADER = 0x8B30;
static const GLenum LT_COMPILE_STATUS  = 0x8B81;
static const GLenum LT_LINK_STATUS     = 0x8B82;
static const GLenum LT_BLEND           = 0x0BE2;
static const GLenum LT_ONE             = 1;
static const GLenum LT_ONE_MINUS_ALPHA = 0x0303;
static const GLboolean LT_TRUE         = 1;
static const GLboolean LT_FALSE        = 0;
static const GLbitfield LT_COLOR_BIT   = 0x4000;
static const GLenum LT_DEPTH_TEST      = 0x0B71;

// GL function pointer types
typedef void (*PFN_genVAO)(GLsizei, GLuint*);
typedef void (*PFN_bindVAO)(GLuint);
typedef void (*PFN_genBuf)(GLsizei, GLuint*);
typedef void (*PFN_bindBuf)(GLenum, GLuint);
typedef void (*PFN_bufData)(GLenum, GLsizeiptr, const GLvoid*, GLenum);
typedef void (*PFN_enableAttr)(GLuint);
typedef void (*PFN_attrPtr)(GLuint, GLint, GLenum, GLboolean, GLsizei, const GLvoid*);
typedef GLuint (*PFN_createShader)(GLenum);
typedef void (*PFN_shaderSource)(GLuint, GLsizei, const GLchar* const*, const GLint*);
typedef void (*PFN_compileShader)(GLuint);
typedef void (*PFN_getShaderiv)(GLuint, GLenum, GLint*);
typedef void (*PFN_getShaderLog)(GLuint, GLsizei, GLsizei*, GLchar*);
typedef GLuint (*PFN_createProgram)(void);
typedef void (*PFN_attachShader)(GLuint, GLuint);
typedef void (*PFN_linkProgram)(GLuint);
typedef void (*PFN_getProgramiv)(GLuint, GLenum, GLint*);
typedef void (*PFN_useProgram)(GLuint);
typedef GLint (*PFN_getUniform)(GLuint, const GLchar*);
typedef void (*PFN_uniform2f)(GLint, GLfloat, GLfloat);
typedef void (*PFN_drawElements)(GLenum, GLsizei, GLenum, const GLvoid*);
typedef void (*PFN_enable)(GLenum);
typedef void (*PFN_disable)(GLenum);
typedef void (*PFN_blendFunc)(GLenum, GLenum);
typedef void (*PFN_viewport)(GLint, GLint, GLsizei, GLsizei);
typedef void (*PFN_clearColor)(GLfloat, GLfloat, GLfloat, GLfloat);
typedef void (*PFN_clear)(GLbitfield);

// GLX types
struct GLXContextOpaque; typedef GLXContextOpaque* GLXContext;
typedef struct __GLXFBConfigRec* GLXFBConfigRec;
typedef GLXFBConfigRec* GLXFBConfig;

typedef GLXContext (*PFN_glXCreateCtxARB)(Display*, GLXFBConfig, GLXContext, int, const int*);
typedef GLXContext (*PFN_glXCreateCtx)(Display*, XVisualInfo*, GLXContext, int);
typedef void (*PFN_glXDestroyCtx)(Display*, GLXContext);
typedef int (*PFN_glXMakeCurrent)(Display*, unsigned long, GLXContext);
typedef void (*PFN_glXSwapBuffers)(Display*, unsigned long);
typedef GLXFBConfigRec** (*PFN_glXChooseFB)(Display*, int, const int*, int*);
typedef XVisualInfo* (*PFN_glXGetVisual)(Display*, GLXFBConfig);
typedef void* (*PFN_glXGetProc)(const unsigned char*);

// Static function pointers
static PFN_genVAO pfn_genVAO;
static PFN_bindVAO pfn_bindVAO;
static PFN_genBuf pfn_genBuf;
static PFN_bindBuf pfn_bindBuf;
static PFN_bufData pfn_bufData;
static PFN_enableAttr pfn_enableAttr;
static PFN_attrPtr pfn_attrPtr;
static PFN_createShader pfn_createShader;
static PFN_shaderSource pfn_shaderSource;
static PFN_compileShader pfn_compileShader;
static PFN_getShaderiv pfn_getShaderiv;
static PFN_getShaderLog pfn_getShaderLog;
static PFN_createProgram pfn_createProgram;
static PFN_attachShader pfn_attachShader;
static PFN_linkProgram pfn_linkProgram;
static PFN_getProgramiv pfn_getProgramiv;
static PFN_useProgram pfn_useProgram;
static PFN_getUniform pfn_getUniform;
static PFN_uniform2f pfn_uniform2f;
static PFN_drawElements pfn_drawElements;
static PFN_enable pfn_enable;
static PFN_disable pfn_disable;
static PFN_blendFunc pfn_blendFunc;
static PFN_viewport pfn_viewport;
static PFN_clearColor pfn_clearColor;
static PFN_clear pfn_clear;
static PFN_glXCreateCtxARB pfn_glXCreateCtxARB;
static PFN_glXCreateCtx pfn_glXCreateCtx;
static PFN_glXDestroyCtx pfn_glXDestroyCtx;
static PFN_glXMakeCurrent pfn_glXMakeCurrent;
static PFN_glXSwapBuffers pfn_glXSwapBuffers;
static PFN_glXChooseFB pfn_glXChooseFB;
static PFN_glXGetVisual pfn_glXGetVisual;
static PFN_glXGetProc pfn_glXGetProc;

static Atom WM_DELETE_WINDOW;

static const char* VERT_SRC = R"GLSL(#version 330 core
layout(location=0) in vec2 a_pos; layout(location=1) in vec4 a_color;
layout(location=2) in vec4 a_uv; layout(location=3) in float a_z; layout(location=4) in uint a_kind;
uniform vec2 u_vp; out vec4 v_color; flat out uint v_kind;
void main() { vec2 ndc = vec2((a_pos.x/u_vp.x)*2.0-1.0, 1.0-(a_pos.y/u_vp.y)*2.0); gl_Position=vec4(ndc,0,1); v_color=a_color; v_kind=a_kind; }
)GLSL";

static const char* FRAG_SRC = R"GLSL(#version 330 core
in vec4 v_color; flat in uint v_kind; out vec4 frag;
void main() { frag = v_color; }
)GLSL";

static GLuint compile_shader(GLenum type, const char* src) {
    GLuint s = pfn_createShader(type);
    pfn_shaderSource(s, 1, &src, nullptr);
    pfn_compileShader(s);
    GLint ok; pfn_getShaderiv(s, LT_COMPILE_STATUS, &ok);
    if (!ok) { char log[512]; pfn_getShaderLog(s, 512, nullptr, log); std::cerr << "shader: " << log << std::endl; return 0; }
    return s;
}

bool GLWindow::init(const GLWindowConfig& config) {
    width_ = config.width; height_ = config.height;
    Display* dpy = XOpenDisplay(nullptr);
    if (!dpy) { std::cerr << "lumen: cannot open X display\n"; return false; }
    display_ = dpy;
    int screen = DefaultScreen(dpy);

    void* gl_handle = dlopen("libGL.so.1", RTLD_LAZY | RTLD_GLOBAL);
    if (!gl_handle) { std::cerr << "lumen: cannot load libGL\n"; return false; }
    void* glx_handle = dlopen("libGLX.so.0", RTLD_LAZY | RTLD_GLOBAL);
    if (!glx_handle) glx_handle = gl_handle;

    pfn_glXGetProc = (PFN_glXGetProc)dlsym(glx_handle, "glXGetProcAddressARB");
    auto load = [&](const char* name) -> void* {
        if (pfn_glXGetProc) { void* p = pfn_glXGetProc((const unsigned char*)name); if (p) return p; }
        return dlsym(gl_handle, name);
    };

    pfn_genVAO = (PFN_genVAO)load("glGenVertexArrays");
    pfn_bindVAO = (PFN_bindVAO)load("glBindVertexArray");
    pfn_genBuf = (PFN_genBuf)load("glGenBuffers");
    pfn_bindBuf = (PFN_bindBuf)load("glBindBuffer");
    pfn_bufData = (PFN_bufData)load("glBufferData");
    pfn_enableAttr = (PFN_enableAttr)load("glEnableVertexAttribArray");
    pfn_attrPtr = (PFN_attrPtr)load("glVertexAttribPointer");
    pfn_createShader = (PFN_createShader)load("glCreateShader");
    pfn_shaderSource = (PFN_shaderSource)load("glShaderSource");
    pfn_compileShader = (PFN_compileShader)load("glCompileShader");
    pfn_getShaderiv = (PFN_getShaderiv)load("glGetShaderiv");
    pfn_getShaderLog = (PFN_getShaderLog)load("glGetShaderInfoLog");
    pfn_createProgram = (PFN_createProgram)load("glCreateProgram");
    pfn_attachShader = (PFN_attachShader)load("glAttachShader");
    pfn_linkProgram = (PFN_linkProgram)load("glLinkProgram");
    pfn_getProgramiv = (PFN_getProgramiv)load("glGetProgramiv");
    pfn_useProgram = (PFN_useProgram)load("glUseProgram");
    pfn_getUniform = (PFN_getUniform)load("glGetUniformLocation");
    pfn_uniform2f = (PFN_uniform2f)load("glUniform2f");
    pfn_drawElements = (PFN_drawElements)load("glDrawElements");
    pfn_enable = (PFN_enable)load("glEnable");
    pfn_disable = (PFN_disable)load("glDisable");
    pfn_blendFunc = (PFN_blendFunc)load("glBlendFunc");
    pfn_viewport = (PFN_viewport)load("glViewport");
    pfn_clearColor = (PFN_clearColor)load("glClearColor");
    pfn_clear = (PFN_clear)load("glClear");

    pfn_glXChooseFB = (PFN_glXChooseFB)dlsym(glx_handle, "glXChooseFBConfig");
    pfn_glXGetVisual = (PFN_glXGetVisual)dlsym(glx_handle, "glXGetVisualFromFBConfig");
    pfn_glXCreateCtx = (PFN_glXCreateCtx)dlsym(glx_handle, "glXCreateContext");
    pfn_glXDestroyCtx = (PFN_glXDestroyCtx)dlsym(glx_handle, "glXDestroyContext");
    pfn_glXMakeCurrent = (PFN_glXMakeCurrent)dlsym(glx_handle, "glXMakeCurrent");
    pfn_glXSwapBuffers = (PFN_glXSwapBuffers)dlsym(glx_handle, "glXSwapBuffers");
    pfn_glXCreateCtxARB = (PFN_glXCreateCtxARB)(pfn_glXGetProc ? pfn_glXGetProc((const unsigned char*)"glXCreateContextAttribsARB") : nullptr);

    int fb_attr[] = {0x801D,1, 0x8013,1, 0x8012,1, 0x8010,1, 1,8, 2,8, 3,8, 4,8, 0};
    int fb_count = 0;
    GLXFBConfigRec** fbc = pfn_glXChooseFB(dpy, screen, fb_attr, &fb_count);
    if (!fbc || fb_count == 0) { std::cerr << "lumen: no FB config\n"; return false; }
    GLXFBConfig best = fbc[0]; XFree(fbc);
    XVisualInfo* vi = pfn_glXGetVisual(dpy, best);
    if (!vi) { std::cerr << "lumen: no visual\n"; return false; }

    Window root = RootWindow(dpy, screen);
    XSetWindowAttributes swa;
    swa.colormap = XCreateColormap(dpy, root, vi->visual, AllocNone);
    swa.event_mask = ExposureMask|KeyPressMask|ButtonPressMask|ButtonReleaseMask|PointerMotionMask|StructureNotifyMask;
    unsigned long win = XCreateWindow(dpy, root, 0, 0, width_, height_, 0, vi->depth, InputOutput, vi->visual, CWColormap|CWEventMask, &swa);
    window_ = reinterpret_cast<void*>(win);
    XStoreName(dpy, win, config.title.c_str());
    XMapWindow(dpy, win);
    WM_DELETE_WINDOW = XInternAtom(dpy, "WM_DELETE_WINDOW", False);
    XSetWMProtocols(dpy, win, &WM_DELETE_WINDOW, 1);

    GLXContext ctx = nullptr;
    if (pfn_glXCreateCtxARB) { int attr[] = {0x2091,3, 0x2092,3, 0x9126,1, 0}; ctx = pfn_glXCreateCtxARB(dpy, best, nullptr, True, attr); }
    if (!ctx) ctx = pfn_glXCreateCtx(dpy, vi, nullptr, True);
    if (!ctx) { std::cerr << "lumen: cannot create GL context\n"; return false; }
    gl_context_ = ctx;
    pfn_glXMakeCurrent(dpy, win, ctx);
    XFree(vi);

    GLuint vs = compile_shader(LT_VERTEX_SHADER, VERT_SRC);
    GLuint fs = compile_shader(LT_FRAGMENT_SHADER, FRAG_SRC);
    shader_program_ = pfn_createProgram();
    pfn_attachShader(shader_program_, vs);
    pfn_attachShader(shader_program_, fs);
    pfn_linkProgram(shader_program_);

    pfn_genVAO(1, &vao_); pfn_bindVAO(vao_);
    pfn_genBuf(1, &vbo_); pfn_genBuf(1, &ibo_);
    pfn_enableAttr(0); pfn_attrPtr(0, 2, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)0);
    pfn_enableAttr(1); pfn_attrPtr(1, 4, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)8);
    pfn_enableAttr(2); pfn_attrPtr(2, 4, LT_UNSIGNED_SHORT, LT_FALSE, sizeof(Vertex), (void*)24);
    pfn_enableAttr(3); pfn_attrPtr(3, 1, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)32);
    pfn_enableAttr(4); pfn_attrPtr(4, 1, LT_UNSIGNED_INT, LT_FALSE, sizeof(Vertex), (void*)36);
    pfn_bindVAO(0);
    pfn_viewport(0, 0, width_, height_);
    pfn_disable(LT_DEPTH_TEST);
    pfn_enable(LT_BLEND);
    pfn_blendFunc(LT_ONE, LT_ONE_MINUS_ALPHA);
    std::cout << "lumen: GPU window ready (" << width_ << "x" << height_ << ")\n";
    return true;
}

void GLWindow::run(std::function<void(Painter&)> on_frame, std::function<void(uint32_t,uint32_t)> on_resize) {
    Display* dpy = (Display*)display_;
    unsigned long win = (unsigned long)window_;
    while (!should_close_) {
        while (XPending(dpy) > 0) {
            XEvent ev; XNextEvent(dpy, &ev);
            if (ev.type == ClientMessage && (Atom)ev.xclient.data.l[0] == WM_DELETE_WINDOW) should_close_ = true;
            if (ev.type == ConfigureNotify) {
                uint32_t w = ev.xconfigure.width, h = ev.xconfigure.height;
                if (w != width_ || h != height_) { width_ = w; height_ = h; pfn_viewport(0, 0, w, h); if (on_resize) on_resize(w, h); }
            }
        }
        Painter painter; on_frame(painter);
        const Mesh& mesh = painter.mesh();
        if (!mesh.vertices.empty()) {
            pfn_bindBuf(LT_ARRAY_BUFFER, vbo_);
            pfn_bufData(LT_ARRAY_BUFFER, mesh.vertices.size() * sizeof(Vertex), mesh.vertices.data(), LT_DYNAMIC_DRAW);
            pfn_bindBuf(LT_ELEMENT_ARRAY, ibo_);
            pfn_bufData(LT_ELEMENT_ARRAY, mesh.indices.size() * sizeof(Index), mesh.indices.data(), LT_DYNAMIC_DRAW);
        }
        pfn_clearColor(0.949f, 0.961f, 0.965f, 1.0f);
        pfn_clear(LT_COLOR_BIT);
        if (!mesh.indices.empty()) {
            pfn_useProgram(shader_program_);
            GLint loc = pfn_getUniform(shader_program_, "u_vp");
            pfn_uniform2f(loc, (float)width_, (float)height_);
            pfn_bindVAO(vao_);
            pfn_drawElements(LT_TRIANGLES, mesh.indices.size(), LT_UNSIGNED_INT, 0);
            pfn_bindVAO(0);
        }
        pfn_glXSwapBuffers(dpy, win);
    }
    cleanup();
}

void GLWindow::cleanup() {
    if (gl_context_) { pfn_glXMakeCurrent((Display*)display_, 0, nullptr); pfn_glXDestroyCtx((Display*)display_, (GLXContext)gl_context_); gl_context_ = nullptr; }
    if (window_) { XDestroyWindow((Display*)display_, (unsigned long)window_); window_ = nullptr; }
    if (display_) { XCloseDisplay((Display*)display_); display_ = nullptr; }
}

GLWindow::~GLWindow() { cleanup(); }

} // namespace lumen

#else // Non-Linux

namespace lumen {
bool GLWindow::init(const GLWindowConfig& c) { std::cerr << "lumen: GPU backend not implemented on this platform.\n"; width_ = c.width; height_ = c.height; return false; }
void GLWindow::run(std::function<void(Painter&)> f, std::function<void(uint32_t,uint32_t)> r) { (void)f; (void)r; }
GLWindow::~GLWindow() {}
void GLWindow::cleanup() {}
} // namespace lumen

#endif
