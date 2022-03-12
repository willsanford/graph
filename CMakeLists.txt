cmake_minimum_required(VERSION 3.21)
project(graph)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(structures/geometry)
include_directories(structures/graph)
include_directories(structures/rbtree)

add_executable(graph
        structures/geometry/point.hpp
        structures/geometry/segment.hpp
        structures/graph/graph.cpp
        structures/graph/graph.hpp
        structures/rbtree/tree.hpp
        bentley_ottmann.cpp
        bentley_ottmann.hpp
        main.cpp structures/geometry/point.cpp structures/geometry/segment.cpp structures/rbtree/node.hpp)
