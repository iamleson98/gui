#include "lumen/render/gl_backend.hpp"
#include "lumen/render/painter.hpp"
#include <iostream>
#include <cstring>
#include <vector>
#include <cstdlib>
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
typedef ptrdiff_t GLintptr, GLsizeiptr;
typedef float GLfloat;
typedef char GLchar;
typedef unsigned char GLboolean;
typedef void GLvoid;
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
// Texture constants for glyph atlas support
static const GLenum LT_TEXTURE0        = 0x84C0;
static const GLenum LT_TEXTURE_2D       = 0x0DE1;
static const GLenum LT_TEXTURE_BINDING_2D = 0x8069;
static const GLenum LT_R8              = 0x8229;
static const GLenum LT_RED             = 0x1903;
static const GLenum LT_UNSIGNED_BYTE   = 0x1401;
static const GLenum LT_NEAREST         = 0x2600;
static const GLenum LT_LINEAR          = 0x2601;
static const GLenum LT_TEXTURE_WRAP_S  = 0x2802;
static const GLenum LT_TEXTURE_WRAP_T  = 0x2803;
static const GLenum LT_CLAMP_TO_EDGE   = 0x812F;
static const GLenum LT_TEXTURE_MAG_FILTER = 0x2800;
static const GLenum LT_TEXTURE_MIN_FILTER = 0x2801;
static const GLenum LT_UNPACK_ALIGNMENT = 0x0CF5;

// GLX constants for FB config attributes
static const int GLX_X_RENDERABLE      = 0x801D;
static const int GLX_DRAWABLE_TYPE     = 0x8010;
static const int GLX_RENDER_TYPE       = 0x8011;
static const int GLX_X_VISUAL_TYPE     = 0x8013;
static const int GLX_TRUE_COLOR        = 0x8002;
static const int GLX_DOUBLEBUFFER      = 0x0004;
static const int GLX_RED_SIZE          = 1;
static const int GLX_GREEN_SIZE        = 2;
static const int GLX_BLUE_SIZE         = 3;
static const int GLX_ALPHA_SIZE        = 4;
static const int GLX_RGBA_BIT          = 0x00000001;
static const int GLX_WINDOW_BIT        = 0x00000001;
static const int GLX_CONTEXT_MAJOR_VERSION_ARB = 0x2091;
static const int GLX_CONTEXT_MINOR_VERSION_ARB = 0x2092;
static const int GLX_CONTEXT_PROFILE_MASK_ARB  = 0x9126;
static const int GLX_CONTEXT_CORE_PROFILE_BIT_ARB = 0x00000001;

// GL function pointer types
typedef void (*PFN_genVAO)(GLsizei, GLuint*);
typedef void (*PFN_bindVAO)(GLuint);
typedef void (*PFN_genBuf)(GLsizei, GLuint*);
typedef void (*PFN_bindBuf)(GLenum, GLuint);
typedef void (*PFN_bufData)(GLenum, GLsizeiptr, const GLvoid*, GLenum);
typedef void (*PFN_bufSubData)(GLenum, GLintptr, GLsizeiptr, const GLvoid*);
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
// Texture functions
typedef void (*PFN_activeTexture)(GLenum);
typedef void (*PFN_genTextures)(GLsizei, GLuint*);
typedef void (*PFN_bindTexture)(GLenum, GLuint);
typedef void (*PFN_texImage2D)(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, const GLvoid*);
typedef void (*PFN_texParameteri)(GLenum, GLenum, GLint);
typedef void (*PFN_pixelStorei)(GLenum, GLint);

// GLX types
struct GLXContextOpaque; typedef GLXContextOpaque* GLXContext;
struct __GLXFBConfigRec;
typedef __GLXFBConfigRec* GLXFBConfig;

typedef GLXContext (*PFN_glXCreateCtxARB)(Display*, GLXFBConfig, GLXContext, int, const int*);
typedef GLXContext (*PFN_glXCreateCtx)(Display*, XVisualInfo*, GLXContext, int);
typedef void (*PFN_glXDestroyCtx)(Display*, GLXContext);
typedef int (*PFN_glXMakeCurrent)(Display*, unsigned long, GLXContext);
typedef void (*PFN_glXSwapBuffers)(Display*, unsigned long);
typedef GLXFBConfig* (*PFN_glXChooseFB)(Display*, int, const int*, int*);
typedef XVisualInfo* (*PFN_glXGetVisual)(Display*, GLXFBConfig);
typedef void* (*PFN_glXGetProc)(const unsigned char*);
typedef const char* (*PFN_glXQueryExt)(Display*, int*, int*);
typedef const char* (*PFN_glXQueryVersion)(Display*, int*, int*);

// Static function pointers
static PFN_genVAO pfn_genVAO;
static PFN_bindVAO pfn_bindVAO;
static PFN_genBuf pfn_genBuf;
static PFN_bindBuf pfn_bindBuf;
static PFN_bufData pfn_bufData;
static PFN_bufSubData pfn_bufSubData;
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
static PFN_activeTexture pfn_activeTexture;
static PFN_genTextures pfn_genTextures;
static PFN_bindTexture pfn_bindTexture;
static PFN_texImage2D pfn_texImage2D;
static PFN_texParameteri pfn_texParameteri;
static PFN_pixelStorei pfn_pixelStorei;
static PFN_glXCreateCtxARB pfn_glXCreateCtxARB;
static PFN_glXCreateCtx pfn_glXCreateCtx;
static PFN_glXDestroyCtx pfn_glXDestroyCtx;
static PFN_glXMakeCurrent pfn_glXMakeCurrent;
static PFN_glXSwapBuffers pfn_glXSwapBuffers;
static PFN_glXChooseFB pfn_glXChooseFB;
static PFN_glXGetVisual pfn_glXGetVisual;
static PFN_glXGetProc pfn_glXGetProc;
static PFN_glXQueryExt pfn_glXQueryExt;
static PFN_glXQueryVersion pfn_glXQueryVersion;

static Atom WM_DELETE_WINDOW;
static GLuint glyph_texture_ = 0;
static GLint u_atlas_loc_ = -1;

static const char* VERT_SRC = R"GLSL(#version 330 core
layout(location=0) in vec2 a_pos; layout(location=1) in vec4 a_color;
layout(location=2) in vec4 a_uv; layout(location=3) in float a_z; layout(location=4) in uint a_kind;
uniform vec2 u_vp; uniform vec2 u_atlas_size;
out vec4 v_color; out vec2 v_uv; flat out uint v_kind;
void main() {
    vec2 ndc = vec2((a_pos.x/u_vp.x)*2.0-1.0, 1.0-(a_pos.y/u_vp.y)*2.0);
    gl_Position = vec4(ndc, 0, 1);
    v_color = a_color;
    // Normalize atlas pixel UVs to [0,1].
    v_uv = vec2(a_uv.x / u_atlas_size.x, a_uv.y / u_atlas_size.y);
    v_kind = a_kind;
}
)GLSL";

static const char* FRAG_SRC = R"GLSL(#version 330 core
in vec4 v_color; in vec2 v_uv; flat in uint v_kind;
uniform sampler2D u_atlas; out vec4 frag;
void main() {
    if (v_kind == 1u) {
        float a = texture(u_atlas, v_uv).r;
        frag = vec4(v_color.rgb, v_color.a * a);
    } else {
        frag = v_color;
    }
}
)GLSL";

static GLuint compile_shader(GLenum type, const char* src) {
    GLuint s = pfn_createShader(type);
    pfn_shaderSource(s, 1, &src, nullptr);
    pfn_compileShader(s);
    GLint ok; pfn_getShaderiv(s, LT_COMPILE_STATUS, &ok);
    if (!ok) { char log[512]; pfn_getShaderLog(s, 512, nullptr, log); std::cerr << "lumen: shader: " << log << std::endl; return 0; }
    return s;
}

// Try to choose an FB config. Returns nullptr on failure. The caller owns
// the returned array (must XFree).
static GLXFBConfig* choose_fb_config(Display* dpy, int screen, bool require_alpha) {
    if (!pfn_glXChooseFB) return nullptr;
    // Standard attributes: double-buffered, RGBA, 8-bit color, optionally alpha.
    std::vector<int> attr;
    attr.push_back(GLX_X_RENDERABLE);   attr.push_back(True);
    attr.push_back(GLX_DRAWABLE_TYPE);  attr.push_back(GLX_WINDOW_BIT);
    attr.push_back(GLX_RENDER_TYPE);    attr.push_back(GLX_RGBA_BIT);
    attr.push_back(GLX_X_VISUAL_TYPE);  attr.push_back(GLX_TRUE_COLOR);
    attr.push_back(GLX_DOUBLEBUFFER);   attr.push_back(True);
    attr.push_back(GLX_RED_SIZE);       attr.push_back(8);
    attr.push_back(GLX_GREEN_SIZE);     attr.push_back(8);
    attr.push_back(GLX_BLUE_SIZE);      attr.push_back(8);
    if (require_alpha) { attr.push_back(GLX_ALPHA_SIZE); attr.push_back(8); }
    attr.push_back(None);

    int fb_count = 0;
    GLXFBConfig* fbc = pfn_glXChooseFB(dpy, screen, attr.data(), &fb_count);
    if (fbc && fb_count > 0) return fbc;  // caller frees
    if (fbc) XFree(fbc);
    return nullptr;
}

bool GLWindow::init(const GLWindowConfig& config) {
    width_ = config.width; height_ = config.height;
    Display* dpy = XOpenDisplay(nullptr);
    if (!dpy) {
        const char* env = std::getenv("DISPLAY");
        std::cerr << "lumen: cannot open X display (DISPLAY="
                  << (env ? env : "") << ").\n"
                  << "lumen: hints — run inside an X session, or use `xvfb-run -a ./counter`,\n"
                  << "lumen:         or set DISPLAY=:99 after starting `Xvfb :99 -screen 0 1024x768x24 +extension GLX`.\n";
        return false;
    }
    display_ = dpy;
    int screen = DefaultScreen(dpy);

    void* gl_handle = dlopen("libGL.so.1", RTLD_LAZY | RTLD_GLOBAL);
    if (!gl_handle) { std::cerr << "lumen: cannot load libGL.so.1: " << dlerror() << "\n"; return false; }
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
    pfn_bufSubData = (PFN_bufSubData)load("glBufferSubData");
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
    pfn_activeTexture = (PFN_activeTexture)load("glActiveTexture");
    pfn_genTextures = (PFN_genTextures)load("glGenTextures");
    pfn_bindTexture = (PFN_bindTexture)load("glBindTexture");
    pfn_texImage2D = (PFN_texImage2D)load("glTexImage2D");
    pfn_texParameteri = (PFN_texParameteri)load("glTexParameteri");
    pfn_pixelStorei = (PFN_pixelStorei)load("glPixelStorei");

    pfn_glXQueryExt = (PFN_glXQueryExt)dlsym(glx_handle, "glXQueryExtension");
    pfn_glXQueryVersion = (PFN_glXQueryVersion)dlsym(glx_handle, "glXQueryVersion");
    pfn_glXChooseFB = (PFN_glXChooseFB)dlsym(glx_handle, "glXChooseFBConfig");
    pfn_glXGetVisual = (PFN_glXGetVisual)dlsym(glx_handle, "glXGetVisualFromFBConfig");
    pfn_glXCreateCtx = (PFN_glXCreateCtx)dlsym(glx_handle, "glXCreateContext");
    pfn_glXDestroyCtx = (PFN_glXDestroyCtx)dlsym(glx_handle, "glXDestroyContext");
    pfn_glXMakeCurrent = (PFN_glXMakeCurrent)dlsym(glx_handle, "glXMakeCurrent");
    pfn_glXSwapBuffers = (PFN_glXSwapBuffers)dlsym(glx_handle, "glXSwapBuffers");
    pfn_glXCreateCtxARB = (PFN_glXCreateCtxARB)(pfn_glXGetProc ? pfn_glXGetProc((const unsigned char*)"glXCreateContextAttribsARB") : nullptr);

    // Verify GLX is present on the server. Xvfb without the GLX module
    // returns false here — better error message than "no FB config".
    if (pfn_glXQueryExt) {
        int ev_base = 0, err_base = 0;
        if (!pfn_glXQueryExt(dpy, &ev_base, &err_base)) {
            std::cerr << "lumen: GLX extension not available on this X server.\n"
                      << "lumen: if using Xvfb, start it with `+extension GLX` and ensure\n"
                      << "lumen: the mesa GLX modules are installed (libglx-mesa0 / libgl1-mesa-dri).\n";
            return false;
        }
        int major = 0, minor = 0;
        if (pfn_glXQueryVersion) pfn_glXQueryVersion(dpy, &major, &minor);
    }

    // Try with alpha first, then without — some Xvfb/swrast configs don't
    // expose alpha in window FB configs.
    GLXFBConfig* fbc = choose_fb_config(dpy, screen, /*require_alpha=*/true);
    if (!fbc) fbc = choose_fb_config(dpy, screen, /*require_alpha=*/false);
    if (!fbc) {
        std::cerr << "lumen: no suitable GLX FB config found.\n"
                  << "lumen: your X server may not have hardware GL or the mesa swrast driver.\n"
                  << "lumen: install libgl1-mesa-dri and libglx-mesa0, or run on a real X session.\n";
        return false;
    }
    GLXFBConfig best = fbc[0]; XFree(fbc);
    XVisualInfo* vi = pfn_glXGetVisual(dpy, best);
    if (!vi) { std::cerr << "lumen: no X visual for FB config\n"; return false; }

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
    if (pfn_glXCreateCtxARB) {
        int attr[] = {GLX_CONTEXT_MAJOR_VERSION_ARB,3, GLX_CONTEXT_MINOR_VERSION_ARB,3, GLX_CONTEXT_PROFILE_MASK_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB, 0};
        ctx = pfn_glXCreateCtxARB(dpy, best, nullptr, True, attr);
    }
    if (!ctx) ctx = pfn_glXCreateCtx(dpy, vi, nullptr, True);
    if (!ctx) { std::cerr << "lumen: cannot create GL context\n"; XFree(vi); return false; }
    gl_context_ = ctx;
    pfn_glXMakeCurrent(dpy, win, ctx);
    XFree(vi);

    GLuint vs = compile_shader(LT_VERTEX_SHADER, VERT_SRC);
    GLuint fs = compile_shader(LT_FRAGMENT_SHADER, FRAG_SRC);
    shader_program_ = pfn_createProgram();
    pfn_attachShader(shader_program_, vs);
    pfn_attachShader(shader_program_, fs);
    pfn_linkProgram(shader_program_);
    GLint linked = 0; pfn_getProgramiv(shader_program_, LT_LINK_STATUS, &linked);
    if (!linked) { std::cerr << "lumen: shader program failed to link\n"; }

    u_atlas_loc_ = pfn_getUniform(shader_program_, "u_atlas");

    pfn_genVAO(1, &vao_); pfn_bindVAO(vao_);
    pfn_genBuf(1, &vbo_); pfn_genBuf(1, &ibo_);
    pfn_enableAttr(0); pfn_attrPtr(0, 2, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)0);
    pfn_enableAttr(1); pfn_attrPtr(1, 4, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)8);
    pfn_enableAttr(2); pfn_attrPtr(2, 4, LT_UNSIGNED_SHORT, LT_FALSE, sizeof(Vertex), (void*)24);
    pfn_enableAttr(3); pfn_attrPtr(3, 1, LT_FLOAT, LT_FALSE, sizeof(Vertex), (void*)32);
    pfn_enableAttr(4); pfn_attrPtr(4, 1, LT_UNSIGNED_INT, LT_FALSE, sizeof(Vertex), (void*)36);
    pfn_bindVAO(0);

    // Create a 1024x1024 R8 glyph atlas texture.
    pfn_genTextures(1, &glyph_texture_);
    pfn_activeTexture(LT_TEXTURE0);
    pfn_bindTexture(LT_TEXTURE_2D, glyph_texture_);
    pfn_pixelStorei(LT_UNPACK_ALIGNMENT, 1);
    std::vector<uint8_t> zero(1024 * 1024, 0);
    pfn_texImage2D(LT_TEXTURE_2D, 0, LT_R8, 1024, 1024, 0, LT_RED, LT_UNSIGNED_BYTE, zero.data());
    pfn_texParameteri(LT_TEXTURE_2D, LT_TEXTURE_WRAP_S, LT_CLAMP_TO_EDGE);
    pfn_texParameteri(LT_TEXTURE_2D, LT_TEXTURE_WRAP_T, LT_CLAMP_TO_EDGE);
    pfn_texParameteri(LT_TEXTURE_2D, LT_TEXTURE_MIN_FILTER, LT_LINEAR);
    pfn_texParameteri(LT_TEXTURE_2D, LT_TEXTURE_MAG_FILTER, LT_LINEAR);

    pfn_viewport(0, 0, width_, height_);
    pfn_disable(LT_DEPTH_TEST);
    pfn_enable(LT_BLEND);
    pfn_blendFunc(LT_ONE, LT_ONE_MINUS_ALPHA);
    std::cout << "lumen: GPU window ready (" << width_ << "x" << height_ << ")\n";
    return true;
}

void GLWindow::upload_glyph_atlas(const uint8_t* pixels, uint32_t width, uint32_t height) {
    if (!glyph_texture_ || !pfn_bindTexture) return;
    pfn_activeTexture(LT_TEXTURE0);
    pfn_bindTexture(LT_TEXTURE_2D, glyph_texture_);
    pfn_pixelStorei(LT_UNPACK_ALIGNMENT, 1);
    pfn_texImage2D(LT_TEXTURE_2D, 0, LT_R8, width, height, 0, LT_RED, LT_UNSIGNED_BYTE, pixels);
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
        // Clear to the theme background (slate-100).
        pfn_clearColor(0.949f, 0.961f, 0.965f, 1.0f);
        pfn_clear(LT_COLOR_BIT);
        if (!mesh.indices.empty()) {
            pfn_useProgram(shader_program_);
            GLint vp_loc = pfn_getUniform(shader_program_, "u_vp");
            pfn_uniform2f(vp_loc, (float)width_, (float)height_);
            GLint atlas_size_loc = pfn_getUniform(shader_program_, "u_atlas_size");
            if (atlas_size_loc >= 0) pfn_uniform2f(atlas_size_loc, 1024.0f, 1024.0f);
            if (u_atlas_loc_ >= 0) {
                pfn_activeTexture(LT_TEXTURE0);
                pfn_bindTexture(LT_TEXTURE_2D, glyph_texture_);
                // Bind sampler to texture unit 0 via uniform (use program-already-set).
            }
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

#else // Non-Linux (macOS, Windows)

namespace lumen {
// On non-Linux platforms, the GL/X11 GPU backend is not available.
// The software renderer (render/software.cpp) handles output instead,
// rasterizing the mesh to a PNG file. This stub just returns false so
// that app.cpp falls through to the software path.
bool GLWindow::init(const GLWindowConfig& c) { width_ = c.width; height_ = c.height; return false; }
void GLWindow::run(std::function<void(Painter&)> f, std::function<void(uint32_t,uint32_t)> r) { (void)f; (void)r; }
GLWindow::~GLWindow() {}
void GLWindow::cleanup() {}
void GLWindow::upload_glyph_atlas(const uint8_t*, uint32_t, uint32_t) {}
} // namespace lumen

#endif
