#include <iostream>
#include <cstring>
#include "antlr4-runtime.h"
#include "trombone_compiler_visitor.h"
#include "antlr/TromboneLexer.h"
#include "antlr/TromboneParser.h"

#include "antlr/TromboneParser.h"
#include "antlr/TromboneLexer.h"


int main(int argc, const char* argv[]) {
  if (argc < 2 || strcmp(argv[1], "--help") == 0) {
    std::cout << "Usage: " << argv[0] << " <input_file> [output_file]" << std::endl;
    return -1;
  }
  std::ifstream input_file(argv[1]);
  if (!input_file.good()) {
    std::cerr << "Could not open input file: " << argv[1] << std::endl;
    return -1;
  }
  

  antlr4::ANTLRInputStream input_stream(input_file);
  TromboneLexer lexer(&input_stream);
  antlr4::CommonTokenStream tokens(&lexer);
  TromboneParser parser(&tokens);

  TromboneParser::ProgramContext *tree = parser.program();

  trombone_compiler_visitor visitor = argc > 2 ? trombone_compiler_visitor(argv[2]) : trombone_compiler_visitor();
  visitor.visitProgram(tree);

  return 0;
}
