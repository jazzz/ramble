cmake_minimum_required(VERSION 3.10)
project(cpp_ramble)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

Include(FetchContent)

FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.8.0 # or a later release
)
FetchContent_MakeAvailable(Catch2)


set(SOURCES test.cpp include/ramble.hpp)

add_executable(tests test.cpp)
target_link_libraries(tests PRIVATE Catch2::Catch2WithMain)
target_include_directories(tests PRIVATE "$ENV{RAMBLE_GENERATED_DIRECTORY}")
message(STATUS "Adding to searchpath: $ENV{RAMBLE_GENERATED_DIRECTORY}")

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)
catch_discover_tests(tests)