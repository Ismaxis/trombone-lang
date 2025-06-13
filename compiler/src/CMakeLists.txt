file(GLOB_RECURSE ANTLR_GEN_SOURCES "antlr/*.cpp")

add_executable(trombone-compiler
    main.cpp

    ${ANTLR_GEN_SOURCES}
)

target_link_libraries(trombone-compiler antlr4_static)
