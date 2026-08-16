CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Wpedantic -Wno-unused-parameter -O2 -Iinclude

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    GL_LIBS = -l:libGL.so.1 -l:libX11.so.6 -ldl
else ifeq ($(UNAME_S),Darwin)
    GL_LIBS = -framework OpenGL -framework Cocoa
else
    GL_LIBS = -lopengl32 -lgdi32
endif
LDFLAGS = $(GL_LIBS)

LIB_SRCS = src/core/color.cpp src/core/id.cpp src/style/style.cpp src/layout/layout.cpp src/render/mesh.cpp src/render/painter.cpp src/render/gl_backend.cpp src/event/event.cpp src/widget/widget.cpp src/platform/app.cpp

all: liblumen.a counter gallery stress themes

liblumen.a: $(LIB_SRCS:.cpp=.o)
	ar rcs $@ $^

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

counter: examples/counter.cpp liblumen.a
	$(CXX) $(CXXFLAGS) $< -L. -llumen $(LDFLAGS) -o $@
gallery: examples/gallery.cpp liblumen.a
	$(CXX) $(CXXFLAGS) $< -L. -llumen $(LDFLAGS) -o $@
stress: examples/stress.cpp liblumen.a
	$(CXX) $(CXXFLAGS) $< -L. -llumen $(LDFLAGS) -o $@
themes: examples/themes.cpp liblumen.a
	$(CXX) $(CXXFLAGS) $< -L. -llumen $(LDFLAGS) -o $@

run: counter
	./counter

test:
	@echo "Tests require separate test files (see tests/ directory)"

clean:
	rm -f *.o src/*.o src/*/*.o counter gallery stress themes liblumen.a

.PHONY: all run test clean
