add_executable(tests test.cpp)

target_link_libraries(tests PRIVATE Catch2::Catch2WithMain)
target_include_directories(tests PRIVATE ${Catch2_SOURCE_DIR}/include)

include(CTest)
include(Catch)
catch_discover_tests(tests)
