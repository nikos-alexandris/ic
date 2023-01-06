cmake_minimum_required(VERSION 3.23)
project(library C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS "-Wall -Wextra -pedantic -Wno-unused-parameter")

add_library(ic STATIC
        include/common.h
        include/lar.h
        include/value.h

        source/common.c
        source/lar.c
        source/value.c
        )
target_include_directories(ic PRIVATE include)