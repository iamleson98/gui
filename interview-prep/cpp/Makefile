CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Wpedantic -O2 -Iinclude
LDFLAGS = -pthread

LIB_SRCS = src/treiber_stack.cpp src/disjoint_set.cpp src/lru_cache.cpp src/edit_distance.cpp src/lis.cpp src/quickselect.cpp

all: run_tests

run_tests: tests/test_main.cpp libinterview.a
	$(CXX) $(CXXFLAGS) tests/test_main.cpp -L. -linterview $(LDFLAGS) -o run_tests

libinterview.a: $(LIB_SRCS:.cpp=.o)
	ar rcs $@ $^

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

test: run_tests
	./run_tests

clean:
	rm -f *.o src/*.o run_tests libinterview.a

.PHONY: all test clean
