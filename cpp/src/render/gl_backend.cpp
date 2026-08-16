#include "lumen/render/gl_backend.hpp"
#include "lumen/render/painter.hpp"
#include <iostream>
#include <cstring>
#include <dlfcn.h>

#if defined(__linux__) || defined(__unix__)
#include <X11/Xlib.h>
#include <X11/Xutil.h>

// OpenGL types and constants
typedef unsigned int GLenum, GLuint; typedef int GLint, GLsizei; typedef float GLfloat; typedef char GLchar;
typedef unsigned char GLboolean; typedef void GLvoid; typedef ptrdiff_t GLsizeiptr;
#define GL_FLOAT 0x1406
#define GL_UNSIGNED_SHORT 0x1403
#define GL_UNSIGNED_INT 0x1405
#define GL_TRIANGLES 0x0004
#define GL_ARRAY_BUFFER 0x8892
#define GL_ELEMENT_ARRAY_BUFFER 0x8893
#define GL_DYNAMIC_DRAW 0x88E8
#define GL_VERTEX_SHADER 0x8B31
#define GL_FRAGMENT_SHADER 0x8B30
#define GL_COMPILE_STATUS 0x8B81
#define GL_LINK_STATUS 0x8B82
#define GL_BLEND 0x0BE2
#define GL_ONE 1
#define GL_ONE_MINUS_SRC_ALPHA 0x0303
#define GL_TRUE 1
#define GL_FALSE 0
#define GL_COLOR_BUFFER_BIT 0x4000
#define GL_DEPTH_TEST 0x0B71

// GL function pointers
typedef void (*PFN_glGenVertexArrays)(GLsizei, GLuint*);
typedef void (*PFN_glBindVertexArray)(GLuint);
typedef void (*PFN_glGenBuffers)(GLsizei, GLuint*);
typedef void (*PFN_glBindBuffer)(GLenum, GLuint);
typedef void (*PFN_glBufferData)(GLenum, GLsizeiptr, const GLvoid*, GLenum);
typedef void (*PFN_glEnableVertexAttribArray)(GLuint);
typedef void (*PFN_glVertexAttribPointer)(GLuint, GLint, GLenum, GLboolean, GLsizei, const GLvoid*);
typedef GLuint (*PFN_glCreateShader)(GLenum);
typedef void (*PFN_glShaderSource)(GLuint, GLsizei, const GLchar* const*, const GLint*);
typedef void (*PFN_glCompileShader)(GLuint);
typedef void (*PFN_glGetShaderiv)(GLuint, GLenum, GLint*);
typedef void (*PFN_glGetShaderInfoLog)(GLuint, GLsizei, GLsizei*, GLchar*);
typedef GLuint (*PFN_glCreateProgram)(void);
typedef void (*PFN_glAttachShader)(GLuint, GLuint);
typedef void (*PFN_glLinkProgram)(GLuint);
typedef void (*PFN_glGetProgramiv)(GLuint, GLenum, GLint*);
typedef void (*PFN_glUseProgram)(GLuint);
typedef GLint (*PFN_glGetUniformLocation)(GLuint, const GLchar*);
typedef void (*PFN_glUniform2f)(GLint, GLfloat, GLfloat);
typedef void (*PFN_glDrawElements)(GLenum, GLsizei, GLenum, const GLvoid*);
typedef void (*PFN_glEnable)(GLenum);
typedef void (*PFN_glDisable)(GLenum);
typedef void (*PFN_glBlendFunc)(GLenum, GLenum);
typedef void (*PFN_glViewport)(GLint, GLint, GLsizei, GLsizei);
typedef void (*PFN_glClearColor)(GLfloat, GLfloat, GLfloat, GLfloat);
typedef void (*PFN_glClear)(GLbitfield);

// GLX
typedef void GLXContext;
typedef struct __GLXFBConfigRec* GLXFBConfig;
typedef GLXContext (*PFN_glXCreateContextAttribsARB)(::Display*, GLXFBConfig, GLXContext, int, const int*);
typedef GLXContext (*PFN_glXCreateContext)(::Display*, void*, GLXContext, int);
typedef void (*PFN_glXDestroyContext)(::Display*, GLXContext);
typedef int (*PFN_glXMakeCurrent)(::Display*, unsigned long, GLXContext);
typedef void (*PFN_glXSwapBuffers)(::Display*, unsigned long);
typedef GLXFBConfig* (*PFN_glXChooseFBConfig)(::Display*, int, const int*, int*);
typedef void* (*PFN_glXGetVisualFromFBConfig)(::Display*, GLXFBConfig);
typedef void* (*PFN_glXGetProcAddressARB)(const unsigned char*);

static PFN_glGenVertexArrays glGenVertexArrays;
static PFN_glBindVertexArray glBindVertexArray;
static PFN_glGenBuffers glGenBuffers;
static PFN_glBindBuffer glBindBuffer;
static PFN_glBufferData glBufferData;
static PFN_glEnableVertexAttribArray glEnableVertexAttribArray;
static PFN_glVertexAttribPointer glVertexAttribPointer;
static PFN_glCreateShader glCreateShader;
static PFN_glShaderSource glShaderSource;
static PFN_glCompileShader glCompileShader;
static PFN_glGetShaderiv glGetShaderiv;
static PFN_glGetShaderInfoLog glGetShaderInfoLog;
static PFN_glCreateProgram glCreateProgram;
static PFN_glAttachShader glAttachShader;
static PFN_glLinkProgram glLinkProgram;
static PFN_glGetProgramiv glGetProgramiv;
static PFN_glUseProgram glUseProgram;
static PFN_glGetUniformLocation glGetUniformLocation;
static PFN_glUniform2f glUniform2f;
static PFN_glDrawElements glDrawElements;
static PFN_glEnable glEnable;
static PFN_glDisable glDisable;
static PFN_glBlendFunc glBlendFunc;
static PFN_glViewport glViewport;
static PFN_glClearColor glClearColor;
static PFN_glClear glClear;
static PFN_glXCreateContextAttribsARB glXCreateContextAttribsARB;
static PFN_glXCreateContext glXCreateContext;
static PFN_glXDestroyContext glXDestroyContext;
static PFN_glXMakeCurrent glXMakeCurrent;
static PFN_glXSwapBuffers glXSwapBuffers;
static PFN_glXChooseFBConfig glXChooseFBConfig;
static PFN_glXGetVisualFromFBConfig glXGetVisualFromFBConfig;
static PFN_glXGetProcAddressARB glXGetProcAddressARB;

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
    GLuint s = glCreateShader(type); glShaderSource(s, 1, &src, nullptr); glCompileShader(s);
    GLint ok; glGetShaderiv(s, GL_COMPILE_STATUS, &ok);
    if(!ok) { char log[512]; glGetShaderInfoLog(s, 512, nullptr, log); std::cerr << "shader err: " << log << std::endl; return 0; }
    return s;
}

bool GLWindow::init(const GLWindowConfig& config) {
    width_ = config.width; height_ = config.height;
    ::Display* dpy = XOpenDisplay(nullptr);
    if(!dpy) { std::cerr << "lumen: cannot open X display\n"; return false; }
    display_ = dpy;
    int screen = DefaultScreen(dpy);

    void* gl_handle = dlopen("libGL.so.1", RTLD_LAZY|RTLD_GLOBAL);
    if(!gl_handle) { std::cerr << "lumen: cannot load libGL\n"; return false; }
    void* glx_handle = dlopen("libGLX.so.0", RTLD_LAZY|RTLD_GLOBAL);
    if(!glx_handle) glx_handle = gl_handle;

    glXGetProcAddressARB = (PFN_glXGetProcAddressARB)dlsym(glx_handle, "glXGetProcAddressARB");
    #define L(name) name = (PFN_##name)(glXGetProcAddressARB ? glXGetProcAddressARB((const unsigned char*)#name) : dlsym(gl_handle, #name))
    L(glGenVertexArrays); L(glBindVertexArray); L(glGenBuffers); L(glBindBuffer); L(glBufferData);
    L(glEnableVertexAttribArray); L(glVertexAttribPointer); L(glCreateShader); L(glShaderSource);
    L(glCompileShader); L(glGetShaderiv); L(glGetShaderInfoLog); L(glCreateProgram); L(glAttachShader);
    L(glLinkProgram); L(glGetProgramiv); L(glUseProgram); L(glGetUniformLocation); L(glUniform2f);
    L(glDrawElements); L(glEnable); L(glDisable); L(glBlendFunc); L(glViewport); L(glClearColor); L(glClear);
    #undef L
    glXChooseFBConfig = (PFN_glXChooseFBConfig)dlsym(glx_handle, "glXChooseFBConfig");
    glXGetVisualFromFBConfig = (PFN_glXGetVisualFromFBConfig)dlsym(glx_handle, "glXGetVisualFromFBConfig");
    glXCreateContext = (PFN_glXCreateContext)dlsym(glx_handle, "glXCreateContext");
    glXDestroyContext = (PFN_glXDestroyContext)dlsym(glx_handle, "glXDestroyContext");
    glXMakeCurrent = (PFN_glXMakeCurrent)dlsym(glx_handle, "glXMakeCurrent");
    glXSwapBuffers = (PFN_glXSwapBuffers)dlsym(glx_handle, "glXSwapBuffers");
    glXCreateContextAttribsARB = (PFN_glXCreateContextAttribsARB)(glXGetProcAddressARB ? glXGetProcAddressARB((const unsigned char*)"glXCreateContextAttribsARB") : nullptr);

    int fb_attr[] = {0x801D,1, 0x8013,1, 0x8012,1, 0x8010,1, 1,8, 2,8, 3,8, 4,8, 0};
    int fb_count = 0;
    GLXFBConfig* fbc = glXChooseFBConfig(dpy, screen, fb_attr, &fb_count);
    if(!fbc || fb_count==0) { std::cerr << "lumen: no FB config\n"; return false; }
    GLXFBConfig best = fbc[0]; XFree(fbc);
    XVisualInfo* vi = (XVisualInfo*)glXGetVisualFromFBConfig(dpy, best);
    if(!vi) { std::cerr << "lumen: no visual\n"; return false; }
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
    if(glXCreateContextAttribsARB) { int attr[] = {0x2091,3, 0x2092,3, 0x9126,1, 0}; ctx = glXCreateContextAttribsARB(dpy, best, nullptr, True, attr); }
    if(!ctx) ctx = glXCreateContext(dpy, vi, nullptr, True);
    if(!ctx) { std::cerr << "lumen: cannot create GL context\n"; return false; }
    gl_context_ = ctx;
    glXMakeCurrent(dpy, win, ctx);
    XFree(vi);

    GLuint vs = compile_shader(GL_VERTEX_SHADER, VERT_SRC);
    GLuint fs = compile_shader(GL_FRAGMENT_SHADER, FRAG_SRC);
    shader_program_ = glCreateProgram();
    glAttachShader(shader_program_, vs); glAttachShader(shader_program_, fs);
    glLinkProgram(shader_program_);
    glGenVertexArrays(1, &vao_); glBindVertexArray(vao_);
    glGenBuffers(1, &vbo_); glGenBuffers(1, &ibo_);
    glEnableVertexAttribArray(0); glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, sizeof(Vertex), (void*)0);
    glEnableVertexAttribArray(1); glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE, sizeof(Vertex), (void*)8);
    glEnableVertexAttribArray(2); glVertexAttribPointer(2, 4, GL_UNSIGNED_SHORT, GL_FALSE, sizeof(Vertex), (void*)24);
    glEnableVertexAttribArray(3); glVertexAttribPointer(3, 1, GL_FLOAT, GL_FALSE, sizeof(Vertex), (void*)32);
    glEnableVertexAttribArray(4); glVertexAttribPointer(4, 1, GL_UNSIGNED_INT, GL_FALSE, sizeof(Vertex), (void*)36);
    glBindVertexArray(0);
    glViewport(0, 0, width_, height_);
    glDisable(GL_DEPTH_TEST); glEnable(GL_BLEND); glBlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
    std::cout << "lumen: GPU window ready (" << width_ << "x" << height_ << ")\n";
    return true;
}

void GLWindow::run(std::function<void(Painter&)> on_frame, std::function<void(uint32_t,uint32_t)> on_resize) {
    ::Display* dpy = (::Display*)display_;
    unsigned long win = (unsigned long)window_;
    while(!should_close_) {
        while(XPending(dpy) > 0) { XEvent ev; XNextEvent(dpy, &ev);
            if(ev.type==ClientMessage && (Atom)ev.xclient.data.l[0]==WM_DELETE_WINDOW) should_close_ = true;
            if(ev.type==ConfigureNotify) { uint32_t w=ev.xconfigure.width, h=ev.xconfigure.height; if(w!=width_||h!=height_) { width_=w; height_=h; glViewport(0,0,w,h); if(on_resize) on_resize(w,h); } }
        }
        Painter painter; on_frame(painter);
        const Mesh& mesh = painter.mesh();
        if(!mesh.vertices.empty()) {
            glBindBuffer(GL_ARRAY_BUFFER, vbo_); glBufferData(GL_ARRAY_BUFFER, mesh.vertices.size()*sizeof(Vertex), mesh.vertices.data(), GL_DYNAMIC_DRAW);
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ibo_); glBufferData(GL_ELEMENT_ARRAY_BUFFER, mesh.indices.size()*sizeof(Index), mesh.indices.data(), GL_DYNAMIC_DRAW);
        }
        glClearColor(0.949f, 0.961f, 0.965f, 1.0f); glClear(GL_COLOR_BUFFER_BIT);
        if(!mesh.indices.empty()) {
            glUseProgram(shader_program_);
            GLint loc = glGetUniformLocation(shader_program_, "u_vp");
            glUniform2f(loc, (float)width_, (float)height_);
            glBindVertexArray(vao_);
            glDrawElements(GL_TRIANGLES, mesh.indices.size(), GL_UNSIGNED_INT, 0);
            glBindVertexArray(0);
        }
        glXSwapBuffers(dpy, win);
    }
    cleanup();
}

void GLWindow::cleanup() {
    if(gl_context_) { glXMakeCurrent((::Display*)display_, 0, nullptr); glXDestroyContext((::Display*)display_, (GLXContext)gl_context_); gl_context_=nullptr; }
    if(window_) { XDestroyWindow((::Display*)display_, (unsigned long)window_); window_=nullptr; }
    if(display_) { XCloseDisplay((::Display*)display_); display_=nullptr; }
}
GLWindow::~GLWindow() { cleanup(); }

#else // Non-Linux
bool GLWindow::init(const GLWindowConfig& c) { std::cerr << "lumen: GPU backend not implemented on this platform.\n"; width_=c.width; height_=c.height; return false; }
void GLWindow::run(std::function<void(Painter&)> f, std::function<void(uint32_t,uint32_t)> r) { (void)f; (void)r; }
GLWindow::~GLWindow() {} void GLWindow::cleanup() {}
#endif

} // namespace lumen
