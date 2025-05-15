cmake_minimum_required(VERSION 3.19)
project(trombone-compiler LANGUAGES CXX)

execute_process(
  COMMAND conan profile show default
  RESULT_VARIABLE profile_result
  OUTPUT_QUIET
  ERROR_QUIET
)

if(NOT profile_result EQUAL 0)
  message(STATUS "Conan profile not found, running conan profile detect")
  execute_process(
    COMMAND conan profile detect --force
    RESULT_VARIABLE detect_result
  )
  if(NOT detect_result EQUAL 0)
    message(FATAL_ERROR "Failed to run conan profile detect")
  endif()
else()
  message(STATUS "Conan profile already exists")
endif()

message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

execute_process(
  COMMAND conan install ${CMAKE_SOURCE_DIR}
          --output-folder=${CMAKE_BINARY_DIR}
          --build=missing
          -s build_type=${CMAKE_BUILD_TYPE}
  WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
  RESULT_VARIABLE conan_result
)

if(NOT conan_result EQUAL 0)
  message(FATAL_ERROR "Conan install failed.")
endif()

set(_toolchain_file "${CMAKE_BINARY_DIR}/build/${CMAKE_BUILD_TYPE}/generators/conan_toolchain.cmake")
include(${_toolchain_file} OPTIONAL RESULT_VARIABLE toolchain_result)

if(toolchain_result STREQUAL "NOTFOUND")
  message(FATAL_ERROR "Conan toolchain not found at ${_toolchain_file}")
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(ANTLR4_INCLUDE_DIRS /usr/local/include/antlr4-runtime/)

find_package(antlr4-runtime REQUIRED)
include_directories(${ANTLR4_INCLUDE_DIRS})

find_package(antlr4-runtime REQUIRED)
find_package(Catch2 REQUIRED)

add_subdirectory(src)
add_subdirectory(tests)

include(CTest)
enable_testing()
