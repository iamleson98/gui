CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Wpedantic -O2 -Iinclude
LDFLAGS = -pthread

# Collect all header-only implementations
CPP_DIRS = concurrency datastructures algorithms sql systemdesign memory performance security networking

all: run_tests

run_tests: tests/test_main.cpp
	$(CXX) $(CXXFLAGS) tests/test_main.cpp $(LDFLAGS) -o run_tests

test: run_tests
	./run_tests

clean:
	rm -f run_tests

.PHONY: all test clean
