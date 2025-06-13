#include <iostream>

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

  antlr4::tree::ParseTree *tree = parser.program();
  LoggableListener listener;
  antlr4::tree::ParseTreeWalker::DEFAULT.walk(&listener, tree);

  return 0;
}
