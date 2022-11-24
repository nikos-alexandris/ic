cmake_minimum_required(VERSION 3.24)
project(runtime C)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_FLAGS "-Wall -Wextra -pedantic")

add_executable(runtime source/main.c include/world.h source/world.c include/common.h include/value.h source/value.c source/common.c source/operators.c include/operators.h)
target_include_directories(runtime PRIVATE include)