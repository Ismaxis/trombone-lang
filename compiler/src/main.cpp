#include <iostream>
#include "antlr4-runtime.h"
#include "trombone_compiler_visitor.h"
#include "antlr/TromboneLexer.h"
#include "antlr/TromboneParser.h"

#include "antlr/TromboneBaseListener.h"
#include "antlr/TromboneParser.h"
#include "antlr/TromboneLexer.h"

namespace {

class LoggableListener : public TromboneBaseListener {
public:
  void enterEveryRule(antlr4::ParserRuleContext* ctx) override {
    std::cout << ctx->toStringTree(true) << '\n';
  }
};

}

int main(int argc, const char* argv[]) {
  std::ifstream input_file(argv[1]);
  if (!input_file.good()) {
    return -1;
  }

  antlr4::ANTLRInputStream input_stream(input_file);
  TromboneLexer lexer(&input_stream);
  antlr4::CommonTokenStream tokens(&lexer);
  TromboneParser parser(&tokens);

  TromboneParser::ProgramContext *tree = parser.program();
  trombone_compiler_visitor visitor;
  visitor.visitProgram(tree);

  return 0;
}
