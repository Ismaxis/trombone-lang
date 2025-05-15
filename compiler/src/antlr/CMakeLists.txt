add_library(antlr STATIC
    TromboneLexer.cpp
    TromboneParser.cpp
    TromboneVisitor.cpp
    TromboneBaseVisitor.cpp)

target_link_libraries(antlr antlr4_static)

target_include_directories(antlr PUBLIC ${ANTLR4_INCLUDE_DIR})