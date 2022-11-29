cmake_minimum_required(VERSION 3.23)
project(library C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_FLAGS "-Wall -Wextra -pedantic")

add_library(ic STATIC include/world.h source/world.c include/common.h include/value.h source/value.c source/common.c include/lar.h)
target_include_directories(ic PRIVATE include)