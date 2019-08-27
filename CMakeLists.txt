cmake_minimum_required(VERSION 3.0)

project(finalfusion_ffi)

enable_testing()

set(CMAKE_C_FLAGS "-Wall -Werror -Wpedantic -std=c11")

add_subdirectory(finalfusion-ffi)
add_subdirectory(tests)
