cmake_minimum_required(VERSION 3.20.0)
project(win32-cpp-test VERSION 0.1.0 LANGUAGES C CXX)

add_executable(win32-cpp-test main.cpp)

target_compile_features(win32-cpp-test PRIVATE cxx_std_17)