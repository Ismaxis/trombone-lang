
// Generated from ../antlr/Trombone.g4 by ANTLR 4.13.2


#include "TromboneVisitor.h"

#include "TromboneParser.h"


using namespace antlrcpp;

using namespace antlr4;

namespace {

struct TromboneParserStaticData final {
  TromboneParserStaticData(std::vector<std::string> ruleNames,
                        std::vector<std::string> literalNames,
                        std::vector<std::string> symbolicNames)
      : ruleNames(std::move(ruleNames)), literalNames(std::move(literalNames)),
        symbolicNames(std::move(symbolicNames)),
        vocabulary(this->literalNames, this->symbolicNames) {}

  TromboneParserStaticData(const TromboneParserStaticData&) = delete;
  TromboneParserStaticData(TromboneParserStaticData&&) = delete;
  TromboneParserStaticData& operator=(const TromboneParserStaticData&) = delete;
  TromboneParserStaticData& operator=(TromboneParserStaticData&&) = delete;

  std::vector<antlr4::dfa::DFA> decisionToDFA;
  antlr4::atn::PredictionContextCache sharedContextCache;
  const std::vector<std::string> ruleNames;
  const std::vector<std::string> literalNames;
  const std::vector<std::string> symbolicNames;
  const antlr4::dfa::Vocabulary vocabulary;
  antlr4::atn::SerializedATNView serializedATN;
  std::unique_ptr<antlr4::atn::ATN> atn;
};

::antlr4::internal::OnceFlag tromboneParserOnceFlag;
#if ANTLR4_USE_THREAD_LOCAL_CACHE
static thread_local
#endif
std::unique_ptr<TromboneParserStaticData> tromboneParserStaticData = nullptr;

void tromboneParserInitialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  if (tromboneParserStaticData != nullptr) {
    return;
  }
#else
  assert(tromboneParserStaticData == nullptr);
#endif
  auto staticData = std::make_unique<TromboneParserStaticData>(
    std::vector<std::string>{
      "program", "functionDecl", "paramList", "param", "returnType", "type", 
      "block", "statement", "varDecl", "assignment", "arrayAssignment", 
      "returnStmt", "whileStmt", "ifStmt", "funcCall", "argList", "expr"
    },
    std::vector<std::string>{
      "", "'fn'", "'('", "')'", "','", "':'", "'->'", "'int'", "'['", "']'", 
      "'{'", "'}'", "';'", "'let'", "'='", "'return'", "'while'", "'if'", 
      "'else'", "'*'", "'/'", "'+'", "'-'", "'<'", "'>'", "'<='", "'>='", 
      "'=='", "'!='", "'read'", "'print'", "'array'"
    },
    std::vector<std::string>{
      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "IDENTIFIER", 
      "NUMBER", "WS", "COMMENT"
    }
  );
  static const int32_t serializedATNSegment[] = {
  	4,1,35,206,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,6,2,
  	7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,13,2,14,7,
  	14,2,15,7,15,2,16,7,16,1,0,4,0,36,8,0,11,0,12,0,37,1,0,1,0,1,1,1,1,1,
  	1,1,1,3,1,46,8,1,1,1,1,1,3,1,50,8,1,1,1,1,1,1,2,1,2,1,2,5,2,57,8,2,10,
  	2,12,2,60,9,2,1,3,1,3,1,3,1,3,1,4,1,4,1,4,1,5,1,5,1,5,1,5,3,5,73,8,5,
  	1,6,1,6,5,6,77,8,6,10,6,12,6,80,9,6,1,6,1,6,1,7,1,7,1,7,1,7,1,7,1,7,1,
  	7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,3,7,101,8,7,1,8,1,8,1,8,1,8,
  	1,8,1,8,1,8,1,9,1,9,1,9,1,9,1,10,1,10,1,10,1,10,1,10,1,10,1,10,1,11,1,
  	11,1,11,1,12,1,12,1,12,1,12,1,13,1,13,1,13,1,13,1,13,1,13,1,13,1,13,5,
  	13,136,8,13,10,13,12,13,139,9,13,1,13,1,13,3,13,143,8,13,1,14,1,14,1,
  	14,3,14,148,8,14,1,14,1,14,1,15,1,15,1,15,5,15,155,8,15,10,15,12,15,158,
  	9,15,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,
  	1,16,1,16,1,16,1,16,1,16,1,16,1,16,3,16,180,8,16,1,16,1,16,1,16,1,16,
  	1,16,1,16,1,16,1,16,3,16,190,8,16,1,16,1,16,1,16,1,16,1,16,1,16,1,16,
  	1,16,1,16,5,16,201,8,16,10,16,12,16,204,9,16,1,16,0,1,32,17,0,2,4,6,8,
  	10,12,14,16,18,20,22,24,26,28,30,32,0,3,1,0,19,20,1,0,21,22,1,0,23,28,
  	215,0,35,1,0,0,0,2,41,1,0,0,0,4,53,1,0,0,0,6,61,1,0,0,0,8,65,1,0,0,0,
  	10,72,1,0,0,0,12,74,1,0,0,0,14,100,1,0,0,0,16,102,1,0,0,0,18,109,1,0,
  	0,0,20,113,1,0,0,0,22,120,1,0,0,0,24,123,1,0,0,0,26,127,1,0,0,0,28,144,
  	1,0,0,0,30,151,1,0,0,0,32,189,1,0,0,0,34,36,3,2,1,0,35,34,1,0,0,0,36,
  	37,1,0,0,0,37,35,1,0,0,0,37,38,1,0,0,0,38,39,1,0,0,0,39,40,5,0,0,1,40,
  	1,1,0,0,0,41,42,5,1,0,0,42,43,5,32,0,0,43,45,5,2,0,0,44,46,3,4,2,0,45,
  	44,1,0,0,0,45,46,1,0,0,0,46,47,1,0,0,0,47,49,5,3,0,0,48,50,3,8,4,0,49,
  	48,1,0,0,0,49,50,1,0,0,0,50,51,1,0,0,0,51,52,3,12,6,0,52,3,1,0,0,0,53,
  	58,3,6,3,0,54,55,5,4,0,0,55,57,3,6,3,0,56,54,1,0,0,0,57,60,1,0,0,0,58,
  	56,1,0,0,0,58,59,1,0,0,0,59,5,1,0,0,0,60,58,1,0,0,0,61,62,5,32,0,0,62,
  	63,5,5,0,0,63,64,3,10,5,0,64,7,1,0,0,0,65,66,5,6,0,0,66,67,3,10,5,0,67,
  	9,1,0,0,0,68,73,5,7,0,0,69,70,5,8,0,0,70,71,5,7,0,0,71,73,5,9,0,0,72,
  	68,1,0,0,0,72,69,1,0,0,0,73,11,1,0,0,0,74,78,5,10,0,0,75,77,3,14,7,0,
  	76,75,1,0,0,0,77,80,1,0,0,0,78,76,1,0,0,0,78,79,1,0,0,0,79,81,1,0,0,0,
  	80,78,1,0,0,0,81,82,5,11,0,0,82,13,1,0,0,0,83,84,3,16,8,0,84,85,5,12,
  	0,0,85,101,1,0,0,0,86,87,3,18,9,0,87,88,5,12,0,0,88,101,1,0,0,0,89,90,
  	3,20,10,0,90,91,5,12,0,0,91,101,1,0,0,0,92,93,3,28,14,0,93,94,5,12,0,
  	0,94,101,1,0,0,0,95,96,3,22,11,0,96,97,5,12,0,0,97,101,1,0,0,0,98,101,
  	3,24,12,0,99,101,3,26,13,0,100,83,1,0,0,0,100,86,1,0,0,0,100,89,1,0,0,
  	0,100,92,1,0,0,0,100,95,1,0,0,0,100,98,1,0,0,0,100,99,1,0,0,0,101,15,
  	1,0,0,0,102,103,5,13,0,0,103,104,5,32,0,0,104,105,5,5,0,0,105,106,3,10,
  	5,0,106,107,5,14,0,0,107,108,3,32,16,0,108,17,1,0,0,0,109,110,5,32,0,
  	0,110,111,5,14,0,0,111,112,3,32,16,0,112,19,1,0,0,0,113,114,5,32,0,0,
  	114,115,5,8,0,0,115,116,3,32,16,0,116,117,5,9,0,0,117,118,5,14,0,0,118,
  	119,3,32,16,0,119,21,1,0,0,0,120,121,5,15,0,0,121,122,3,32,16,0,122,23,
  	1,0,0,0,123,124,5,16,0,0,124,125,3,32,16,0,125,126,3,12,6,0,126,25,1,
  	0,0,0,127,128,5,17,0,0,128,129,3,32,16,0,129,137,3,12,6,0,130,131,5,18,
  	0,0,131,132,5,17,0,0,132,133,3,32,16,0,133,134,3,12,6,0,134,136,1,0,0,
  	0,135,130,1,0,0,0,136,139,1,0,0,0,137,135,1,0,0,0,137,138,1,0,0,0,138,
  	142,1,0,0,0,139,137,1,0,0,0,140,141,5,18,0,0,141,143,3,12,6,0,142,140,
  	1,0,0,0,142,143,1,0,0,0,143,27,1,0,0,0,144,145,5,32,0,0,145,147,5,2,0,
  	0,146,148,3,30,15,0,147,146,1,0,0,0,147,148,1,0,0,0,148,149,1,0,0,0,149,
  	150,5,3,0,0,150,29,1,0,0,0,151,156,3,32,16,0,152,153,5,4,0,0,153,155,
  	3,32,16,0,154,152,1,0,0,0,155,158,1,0,0,0,156,154,1,0,0,0,156,157,1,0,
  	0,0,157,31,1,0,0,0,158,156,1,0,0,0,159,160,6,16,-1,0,160,161,5,32,0,0,
  	161,162,5,8,0,0,162,163,3,32,16,0,163,164,5,9,0,0,164,190,1,0,0,0,165,
  	190,3,28,14,0,166,167,5,29,0,0,167,168,5,2,0,0,168,190,5,3,0,0,169,170,
  	5,30,0,0,170,171,5,2,0,0,171,172,3,32,16,0,172,173,5,3,0,0,173,190,1,
  	0,0,0,174,175,5,31,0,0,175,176,5,2,0,0,176,179,3,32,16,0,177,178,5,4,
  	0,0,178,180,3,32,16,0,179,177,1,0,0,0,179,180,1,0,0,0,180,181,1,0,0,0,
  	181,182,5,3,0,0,182,190,1,0,0,0,183,184,5,2,0,0,184,185,3,32,16,0,185,
  	186,5,3,0,0,186,190,1,0,0,0,187,190,5,33,0,0,188,190,5,32,0,0,189,159,
  	1,0,0,0,189,165,1,0,0,0,189,166,1,0,0,0,189,169,1,0,0,0,189,174,1,0,0,
  	0,189,183,1,0,0,0,189,187,1,0,0,0,189,188,1,0,0,0,190,202,1,0,0,0,191,
  	192,10,11,0,0,192,193,7,0,0,0,193,201,3,32,16,12,194,195,10,10,0,0,195,
  	196,7,1,0,0,196,201,3,32,16,11,197,198,10,9,0,0,198,199,7,2,0,0,199,201,
  	3,32,16,10,200,191,1,0,0,0,200,194,1,0,0,0,200,197,1,0,0,0,201,204,1,
  	0,0,0,202,200,1,0,0,0,202,203,1,0,0,0,203,33,1,0,0,0,204,202,1,0,0,0,
  	15,37,45,49,58,72,78,100,137,142,147,156,179,189,200,202
  };
  staticData->serializedATN = antlr4::atn::SerializedATNView(serializedATNSegment, sizeof(serializedATNSegment) / sizeof(serializedATNSegment[0]));

  antlr4::atn::ATNDeserializer deserializer;
  staticData->atn = deserializer.deserialize(staticData->serializedATN);

  const size_t count = staticData->atn->getNumberOfDecisions();
  staticData->decisionToDFA.reserve(count);
  for (size_t i = 0; i < count; i++) { 
    staticData->decisionToDFA.emplace_back(staticData->atn->getDecisionState(i), i);
  }
  tromboneParserStaticData = std::move(staticData);
}

}

TromboneParser::TromboneParser(TokenStream *input) : TromboneParser(input, antlr4::atn::ParserATNSimulatorOptions()) {}

TromboneParser::TromboneParser(TokenStream *input, const antlr4::atn::ParserATNSimulatorOptions &options) : Parser(input) {
  TromboneParser::initialize();
  _interpreter = new atn::ParserATNSimulator(this, *tromboneParserStaticData->atn, tromboneParserStaticData->decisionToDFA, tromboneParserStaticData->sharedContextCache, options);
}

TromboneParser::~TromboneParser() {
  delete _interpreter;
}

const atn::ATN& TromboneParser::getATN() const {
  return *tromboneParserStaticData->atn;
}

std::string TromboneParser::getGrammarFileName() const {
  return "Trombone.g4";
}

const std::vector<std::string>& TromboneParser::getRuleNames() const {
  return tromboneParserStaticData->ruleNames;
}

const dfa::Vocabulary& TromboneParser::getVocabulary() const {
  return tromboneParserStaticData->vocabulary;
}

antlr4::atn::SerializedATNView TromboneParser::getSerializedATN() const {
  return tromboneParserStaticData->serializedATN;
}


//----------------- ProgramContext ------------------------------------------------------------------

TromboneParser::ProgramContext::ProgramContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::ProgramContext::EOF() {
  return getToken(TromboneParser::EOF, 0);
}

std::vector<TromboneParser::FunctionDeclContext *> TromboneParser::ProgramContext::functionDecl() {
  return getRuleContexts<TromboneParser::FunctionDeclContext>();
}

TromboneParser::FunctionDeclContext* TromboneParser::ProgramContext::functionDecl(size_t i) {
  return getRuleContext<TromboneParser::FunctionDeclContext>(i);
}


size_t TromboneParser::ProgramContext::getRuleIndex() const {
  return TromboneParser::RuleProgram;
}


std::any TromboneParser::ProgramContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitProgram(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ProgramContext* TromboneParser::program() {
  ProgramContext *_localctx = _tracker.createInstance<ProgramContext>(_ctx, getState());
  enterRule(_localctx, 0, TromboneParser::RuleProgram);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(35); 
    _errHandler->sync(this);
    _la = _input->LA(1);
    do {
      setState(34);
      functionDecl();
      setState(37); 
      _errHandler->sync(this);
      _la = _input->LA(1);
    } while (_la == TromboneParser::T__0);
    setState(39);
    match(TromboneParser::EOF);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- FunctionDeclContext ------------------------------------------------------------------

TromboneParser::FunctionDeclContext::FunctionDeclContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::FunctionDeclContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::BlockContext* TromboneParser::FunctionDeclContext::block() {
  return getRuleContext<TromboneParser::BlockContext>(0);
}

TromboneParser::ParamListContext* TromboneParser::FunctionDeclContext::paramList() {
  return getRuleContext<TromboneParser::ParamListContext>(0);
}

TromboneParser::ReturnTypeContext* TromboneParser::FunctionDeclContext::returnType() {
  return getRuleContext<TromboneParser::ReturnTypeContext>(0);
}


size_t TromboneParser::FunctionDeclContext::getRuleIndex() const {
  return TromboneParser::RuleFunctionDecl;
}


std::any TromboneParser::FunctionDeclContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitFunctionDecl(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::FunctionDeclContext* TromboneParser::functionDecl() {
  FunctionDeclContext *_localctx = _tracker.createInstance<FunctionDeclContext>(_ctx, getState());
  enterRule(_localctx, 2, TromboneParser::RuleFunctionDecl);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(41);
    match(TromboneParser::T__0);
    setState(42);
    match(TromboneParser::IDENTIFIER);
    setState(43);
    match(TromboneParser::T__1);
    setState(45);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::IDENTIFIER) {
      setState(44);
      paramList();
    }
    setState(47);
    match(TromboneParser::T__2);
    setState(49);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__5) {
      setState(48);
      returnType();
    }
    setState(51);
    block();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ParamListContext ------------------------------------------------------------------

TromboneParser::ParamListContext::ParamListContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<TromboneParser::ParamContext *> TromboneParser::ParamListContext::param() {
  return getRuleContexts<TromboneParser::ParamContext>();
}

TromboneParser::ParamContext* TromboneParser::ParamListContext::param(size_t i) {
  return getRuleContext<TromboneParser::ParamContext>(i);
}


size_t TromboneParser::ParamListContext::getRuleIndex() const {
  return TromboneParser::RuleParamList;
}


std::any TromboneParser::ParamListContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitParamList(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ParamListContext* TromboneParser::paramList() {
  ParamListContext *_localctx = _tracker.createInstance<ParamListContext>(_ctx, getState());
  enterRule(_localctx, 4, TromboneParser::RuleParamList);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(53);
    param();
    setState(58);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(54);
      match(TromboneParser::T__3);
      setState(55);
      param();
      setState(60);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ParamContext ------------------------------------------------------------------

TromboneParser::ParamContext::ParamContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::ParamContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::TypeContext* TromboneParser::ParamContext::type() {
  return getRuleContext<TromboneParser::TypeContext>(0);
}


size_t TromboneParser::ParamContext::getRuleIndex() const {
  return TromboneParser::RuleParam;
}


std::any TromboneParser::ParamContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitParam(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ParamContext* TromboneParser::param() {
  ParamContext *_localctx = _tracker.createInstance<ParamContext>(_ctx, getState());
  enterRule(_localctx, 6, TromboneParser::RuleParam);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(61);
    match(TromboneParser::IDENTIFIER);
    setState(62);
    match(TromboneParser::T__4);
    setState(63);
    type();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ReturnTypeContext ------------------------------------------------------------------

TromboneParser::ReturnTypeContext::ReturnTypeContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::TypeContext* TromboneParser::ReturnTypeContext::type() {
  return getRuleContext<TromboneParser::TypeContext>(0);
}


size_t TromboneParser::ReturnTypeContext::getRuleIndex() const {
  return TromboneParser::RuleReturnType;
}


std::any TromboneParser::ReturnTypeContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitReturnType(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ReturnTypeContext* TromboneParser::returnType() {
  ReturnTypeContext *_localctx = _tracker.createInstance<ReturnTypeContext>(_ctx, getState());
  enterRule(_localctx, 8, TromboneParser::RuleReturnType);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(65);
    match(TromboneParser::T__5);
    setState(66);
    type();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- TypeContext ------------------------------------------------------------------

TromboneParser::TypeContext::TypeContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t TromboneParser::TypeContext::getRuleIndex() const {
  return TromboneParser::RuleType;
}


std::any TromboneParser::TypeContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitType(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::TypeContext* TromboneParser::type() {
  TypeContext *_localctx = _tracker.createInstance<TypeContext>(_ctx, getState());
  enterRule(_localctx, 10, TromboneParser::RuleType);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    setState(72);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case TromboneParser::T__6: {
        enterOuterAlt(_localctx, 1);
        setState(68);
        match(TromboneParser::T__6);
        break;
      }

      case TromboneParser::T__7: {
        enterOuterAlt(_localctx, 2);
        setState(69);
        match(TromboneParser::T__7);
        setState(70);
        match(TromboneParser::T__6);
        setState(71);
        match(TromboneParser::T__8);
        break;
      }

    default:
      throw NoViableAltException(this);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- BlockContext ------------------------------------------------------------------

TromboneParser::BlockContext::BlockContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<TromboneParser::StatementContext *> TromboneParser::BlockContext::statement() {
  return getRuleContexts<TromboneParser::StatementContext>();
}

TromboneParser::StatementContext* TromboneParser::BlockContext::statement(size_t i) {
  return getRuleContext<TromboneParser::StatementContext>(i);
}


size_t TromboneParser::BlockContext::getRuleIndex() const {
  return TromboneParser::RuleBlock;
}


std::any TromboneParser::BlockContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitBlock(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::BlockContext* TromboneParser::block() {
  BlockContext *_localctx = _tracker.createInstance<BlockContext>(_ctx, getState());
  enterRule(_localctx, 12, TromboneParser::RuleBlock);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(74);
    match(TromboneParser::T__9);
    setState(78);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 4295204864) != 0)) {
      setState(75);
      statement();
      setState(80);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
    setState(81);
    match(TromboneParser::T__10);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- StatementContext ------------------------------------------------------------------

TromboneParser::StatementContext::StatementContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::VarDeclContext* TromboneParser::StatementContext::varDecl() {
  return getRuleContext<TromboneParser::VarDeclContext>(0);
}

TromboneParser::AssignmentContext* TromboneParser::StatementContext::assignment() {
  return getRuleContext<TromboneParser::AssignmentContext>(0);
}

TromboneParser::ArrayAssignmentContext* TromboneParser::StatementContext::arrayAssignment() {
  return getRuleContext<TromboneParser::ArrayAssignmentContext>(0);
}

TromboneParser::FuncCallContext* TromboneParser::StatementContext::funcCall() {
  return getRuleContext<TromboneParser::FuncCallContext>(0);
}

TromboneParser::ReturnStmtContext* TromboneParser::StatementContext::returnStmt() {
  return getRuleContext<TromboneParser::ReturnStmtContext>(0);
}

TromboneParser::WhileStmtContext* TromboneParser::StatementContext::whileStmt() {
  return getRuleContext<TromboneParser::WhileStmtContext>(0);
}

TromboneParser::IfStmtContext* TromboneParser::StatementContext::ifStmt() {
  return getRuleContext<TromboneParser::IfStmtContext>(0);
}


size_t TromboneParser::StatementContext::getRuleIndex() const {
  return TromboneParser::RuleStatement;
}


std::any TromboneParser::StatementContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitStatement(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::StatementContext* TromboneParser::statement() {
  StatementContext *_localctx = _tracker.createInstance<StatementContext>(_ctx, getState());
  enterRule(_localctx, 14, TromboneParser::RuleStatement);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    setState(100);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(83);
      varDecl();
      setState(84);
      match(TromboneParser::T__11);
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(86);
      assignment();
      setState(87);
      match(TromboneParser::T__11);
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(89);
      arrayAssignment();
      setState(90);
      match(TromboneParser::T__11);
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(92);
      funcCall();
      setState(93);
      match(TromboneParser::T__11);
      break;
    }

    case 5: {
      enterOuterAlt(_localctx, 5);
      setState(95);
      returnStmt();
      setState(96);
      match(TromboneParser::T__11);
      break;
    }

    case 6: {
      enterOuterAlt(_localctx, 6);
      setState(98);
      whileStmt();
      break;
    }

    case 7: {
      enterOuterAlt(_localctx, 7);
      setState(99);
      ifStmt();
      break;
    }

    default:
      break;
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- VarDeclContext ------------------------------------------------------------------

TromboneParser::VarDeclContext::VarDeclContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::VarDeclContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::TypeContext* TromboneParser::VarDeclContext::type() {
  return getRuleContext<TromboneParser::TypeContext>(0);
}

TromboneParser::ExprContext* TromboneParser::VarDeclContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}


size_t TromboneParser::VarDeclContext::getRuleIndex() const {
  return TromboneParser::RuleVarDecl;
}


std::any TromboneParser::VarDeclContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitVarDecl(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::VarDeclContext* TromboneParser::varDecl() {
  VarDeclContext *_localctx = _tracker.createInstance<VarDeclContext>(_ctx, getState());
  enterRule(_localctx, 16, TromboneParser::RuleVarDecl);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(102);
    match(TromboneParser::T__12);
    setState(103);
    match(TromboneParser::IDENTIFIER);
    setState(104);
    match(TromboneParser::T__4);
    setState(105);
    type();
    setState(106);
    match(TromboneParser::T__13);
    setState(107);
    expr(0);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- AssignmentContext ------------------------------------------------------------------

TromboneParser::AssignmentContext::AssignmentContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::AssignmentContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::ExprContext* TromboneParser::AssignmentContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}


size_t TromboneParser::AssignmentContext::getRuleIndex() const {
  return TromboneParser::RuleAssignment;
}


std::any TromboneParser::AssignmentContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitAssignment(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::AssignmentContext* TromboneParser::assignment() {
  AssignmentContext *_localctx = _tracker.createInstance<AssignmentContext>(_ctx, getState());
  enterRule(_localctx, 18, TromboneParser::RuleAssignment);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(109);
    match(TromboneParser::IDENTIFIER);
    setState(110);
    match(TromboneParser::T__13);
    setState(111);
    expr(0);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ArrayAssignmentContext ------------------------------------------------------------------

TromboneParser::ArrayAssignmentContext::ArrayAssignmentContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::ArrayAssignmentContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

std::vector<TromboneParser::ExprContext *> TromboneParser::ArrayAssignmentContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::ArrayAssignmentContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}


size_t TromboneParser::ArrayAssignmentContext::getRuleIndex() const {
  return TromboneParser::RuleArrayAssignment;
}


std::any TromboneParser::ArrayAssignmentContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitArrayAssignment(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ArrayAssignmentContext* TromboneParser::arrayAssignment() {
  ArrayAssignmentContext *_localctx = _tracker.createInstance<ArrayAssignmentContext>(_ctx, getState());
  enterRule(_localctx, 20, TromboneParser::RuleArrayAssignment);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(113);
    match(TromboneParser::IDENTIFIER);
    setState(114);
    match(TromboneParser::T__7);
    setState(115);
    expr(0);
    setState(116);
    match(TromboneParser::T__8);
    setState(117);
    match(TromboneParser::T__13);
    setState(118);
    expr(0);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ReturnStmtContext ------------------------------------------------------------------

TromboneParser::ReturnStmtContext::ReturnStmtContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::ExprContext* TromboneParser::ReturnStmtContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}


size_t TromboneParser::ReturnStmtContext::getRuleIndex() const {
  return TromboneParser::RuleReturnStmt;
}


std::any TromboneParser::ReturnStmtContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitReturnStmt(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ReturnStmtContext* TromboneParser::returnStmt() {
  ReturnStmtContext *_localctx = _tracker.createInstance<ReturnStmtContext>(_ctx, getState());
  enterRule(_localctx, 22, TromboneParser::RuleReturnStmt);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(120);
    match(TromboneParser::T__14);
    setState(121);
    expr(0);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- WhileStmtContext ------------------------------------------------------------------

TromboneParser::WhileStmtContext::WhileStmtContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::ExprContext* TromboneParser::WhileStmtContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}

TromboneParser::BlockContext* TromboneParser::WhileStmtContext::block() {
  return getRuleContext<TromboneParser::BlockContext>(0);
}


size_t TromboneParser::WhileStmtContext::getRuleIndex() const {
  return TromboneParser::RuleWhileStmt;
}


std::any TromboneParser::WhileStmtContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitWhileStmt(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::WhileStmtContext* TromboneParser::whileStmt() {
  WhileStmtContext *_localctx = _tracker.createInstance<WhileStmtContext>(_ctx, getState());
  enterRule(_localctx, 24, TromboneParser::RuleWhileStmt);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(123);
    match(TromboneParser::T__15);
    setState(124);
    expr(0);
    setState(125);
    block();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- IfStmtContext ------------------------------------------------------------------

TromboneParser::IfStmtContext::IfStmtContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<TromboneParser::ExprContext *> TromboneParser::IfStmtContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::IfStmtContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}

std::vector<TromboneParser::BlockContext *> TromboneParser::IfStmtContext::block() {
  return getRuleContexts<TromboneParser::BlockContext>();
}

TromboneParser::BlockContext* TromboneParser::IfStmtContext::block(size_t i) {
  return getRuleContext<TromboneParser::BlockContext>(i);
}


size_t TromboneParser::IfStmtContext::getRuleIndex() const {
  return TromboneParser::RuleIfStmt;
}


std::any TromboneParser::IfStmtContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitIfStmt(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::IfStmtContext* TromboneParser::ifStmt() {
  IfStmtContext *_localctx = _tracker.createInstance<IfStmtContext>(_ctx, getState());
  enterRule(_localctx, 26, TromboneParser::RuleIfStmt);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(127);
    match(TromboneParser::T__16);
    setState(128);
    expr(0);
    setState(129);
    block();
    setState(137);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 7, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(130);
        match(TromboneParser::T__17);
        setState(131);
        match(TromboneParser::T__16);
        setState(132);
        expr(0);
        setState(133);
        block(); 
      }
      setState(139);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 7, _ctx);
    }
    setState(142);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__17) {
      setState(140);
      match(TromboneParser::T__17);
      setState(141);
      block();
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- FuncCallContext ------------------------------------------------------------------

TromboneParser::FuncCallContext::FuncCallContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* TromboneParser::FuncCallContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::ArgListContext* TromboneParser::FuncCallContext::argList() {
  return getRuleContext<TromboneParser::ArgListContext>(0);
}


size_t TromboneParser::FuncCallContext::getRuleIndex() const {
  return TromboneParser::RuleFuncCall;
}


std::any TromboneParser::FuncCallContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitFuncCall(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::FuncCallContext* TromboneParser::funcCall() {
  FuncCallContext *_localctx = _tracker.createInstance<FuncCallContext>(_ctx, getState());
  enterRule(_localctx, 28, TromboneParser::RuleFuncCall);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(144);
    match(TromboneParser::IDENTIFIER);
    setState(145);
    match(TromboneParser::T__1);
    setState(147);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 16642998276) != 0)) {
      setState(146);
      argList();
    }
    setState(149);
    match(TromboneParser::T__2);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ArgListContext ------------------------------------------------------------------

TromboneParser::ArgListContext::ArgListContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

std::vector<TromboneParser::ExprContext *> TromboneParser::ArgListContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::ArgListContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}


size_t TromboneParser::ArgListContext::getRuleIndex() const {
  return TromboneParser::RuleArgList;
}


std::any TromboneParser::ArgListContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitArgList(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ArgListContext* TromboneParser::argList() {
  ArgListContext *_localctx = _tracker.createInstance<ArgListContext>(_ctx, getState());
  enterRule(_localctx, 30, TromboneParser::RuleArgList);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(151);
    expr(0);
    setState(156);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(152);
      match(TromboneParser::T__3);
      setState(153);
      expr(0);
      setState(158);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ExprContext ------------------------------------------------------------------

TromboneParser::ExprContext::ExprContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t TromboneParser::ExprContext::getRuleIndex() const {
  return TromboneParser::RuleExpr;
}

void TromboneParser::ExprContext::copyFrom(ExprContext *ctx) {
  ParserRuleContext::copyFrom(ctx);
}

//----------------- ArrayAccessContext ------------------------------------------------------------------

tree::TerminalNode* TromboneParser::ArrayAccessContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::ExprContext* TromboneParser::ArrayAccessContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}

TromboneParser::ArrayAccessContext::ArrayAccessContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::ArrayAccessContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitArrayAccess(this);
  else
    return visitor->visitChildren(this);
}
//----------------- VarReferenceContext ------------------------------------------------------------------

tree::TerminalNode* TromboneParser::VarReferenceContext::IDENTIFIER() {
  return getToken(TromboneParser::IDENTIFIER, 0);
}

TromboneParser::VarReferenceContext::VarReferenceContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::VarReferenceContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitVarReference(this);
  else
    return visitor->visitChildren(this);
}
//----------------- ReadExprContext ------------------------------------------------------------------

TromboneParser::ReadExprContext::ReadExprContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::ReadExprContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitReadExpr(this);
  else
    return visitor->visitChildren(this);
}
//----------------- MulDivContext ------------------------------------------------------------------

std::vector<TromboneParser::ExprContext *> TromboneParser::MulDivContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::MulDivContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}

TromboneParser::MulDivContext::MulDivContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::MulDivContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitMulDiv(this);
  else
    return visitor->visitChildren(this);
}
//----------------- AddSubContext ------------------------------------------------------------------

std::vector<TromboneParser::ExprContext *> TromboneParser::AddSubContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::AddSubContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}

TromboneParser::AddSubContext::AddSubContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::AddSubContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitAddSub(this);
  else
    return visitor->visitChildren(this);
}
//----------------- ParensContext ------------------------------------------------------------------

TromboneParser::ExprContext* TromboneParser::ParensContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}

TromboneParser::ParensContext::ParensContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::ParensContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitParens(this);
  else
    return visitor->visitChildren(this);
}
//----------------- ArrayCreateContext ------------------------------------------------------------------

std::vector<TromboneParser::ExprContext *> TromboneParser::ArrayCreateContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::ArrayCreateContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}

TromboneParser::ArrayCreateContext::ArrayCreateContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::ArrayCreateContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitArrayCreate(this);
  else
    return visitor->visitChildren(this);
}
//----------------- IntLiteralContext ------------------------------------------------------------------

tree::TerminalNode* TromboneParser::IntLiteralContext::NUMBER() {
  return getToken(TromboneParser::NUMBER, 0);
}

TromboneParser::IntLiteralContext::IntLiteralContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::IntLiteralContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitIntLiteral(this);
  else
    return visitor->visitChildren(this);
}
//----------------- CompareContext ------------------------------------------------------------------

std::vector<TromboneParser::ExprContext *> TromboneParser::CompareContext::expr() {
  return getRuleContexts<TromboneParser::ExprContext>();
}

TromboneParser::ExprContext* TromboneParser::CompareContext::expr(size_t i) {
  return getRuleContext<TromboneParser::ExprContext>(i);
}

TromboneParser::CompareContext::CompareContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::CompareContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitCompare(this);
  else
    return visitor->visitChildren(this);
}
//----------------- PrintExprContext ------------------------------------------------------------------

TromboneParser::ExprContext* TromboneParser::PrintExprContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}

TromboneParser::PrintExprContext::PrintExprContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::PrintExprContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitPrintExpr(this);
  else
    return visitor->visitChildren(this);
}
//----------------- FuncCallExprContext ------------------------------------------------------------------

TromboneParser::FuncCallContext* TromboneParser::FuncCallExprContext::funcCall() {
  return getRuleContext<TromboneParser::FuncCallContext>(0);
}

TromboneParser::FuncCallExprContext::FuncCallExprContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::FuncCallExprContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitFuncCallExpr(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::ExprContext* TromboneParser::expr() {
   return expr(0);
}

TromboneParser::ExprContext* TromboneParser::expr(int precedence) {
  ParserRuleContext *parentContext = _ctx;
  size_t parentState = getState();
  TromboneParser::ExprContext *_localctx = _tracker.createInstance<ExprContext>(_ctx, parentState);
  TromboneParser::ExprContext *previousContext = _localctx;
  (void)previousContext; // Silence compiler, in case the context is not used by generated code.
  size_t startState = 32;
  enterRecursionRule(_localctx, 32, TromboneParser::RuleExpr, precedence);

    size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    unrollRecursionContexts(parentContext);
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(189);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 12, _ctx)) {
    case 1: {
      _localctx = _tracker.createInstance<ArrayAccessContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;

      setState(160);
      match(TromboneParser::IDENTIFIER);
      setState(161);
      match(TromboneParser::T__7);
      setState(162);
      expr(0);
      setState(163);
      match(TromboneParser::T__8);
      break;
    }

    case 2: {
      _localctx = _tracker.createInstance<FuncCallExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(165);
      funcCall();
      break;
    }

    case 3: {
      _localctx = _tracker.createInstance<ReadExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(166);
      match(TromboneParser::T__28);
      setState(167);
      match(TromboneParser::T__1);
      setState(168);
      match(TromboneParser::T__2);
      break;
    }

    case 4: {
      _localctx = _tracker.createInstance<PrintExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(169);
      match(TromboneParser::T__29);
      setState(170);
      match(TromboneParser::T__1);
      setState(171);
      expr(0);
      setState(172);
      match(TromboneParser::T__2);
      break;
    }

    case 5: {
      _localctx = _tracker.createInstance<ArrayCreateContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(174);
      match(TromboneParser::T__30);
      setState(175);
      match(TromboneParser::T__1);
      setState(176);
      expr(0);
      setState(179);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == TromboneParser::T__3) {
        setState(177);
        match(TromboneParser::T__3);
        setState(178);
        expr(0);
      }
      setState(181);
      match(TromboneParser::T__2);
      break;
    }

    case 6: {
      _localctx = _tracker.createInstance<ParensContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(183);
      match(TromboneParser::T__1);
      setState(184);
      expr(0);
      setState(185);
      match(TromboneParser::T__2);
      break;
    }

    case 7: {
      _localctx = _tracker.createInstance<IntLiteralContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(187);
      match(TromboneParser::NUMBER);
      break;
    }

    case 8: {
      _localctx = _tracker.createInstance<VarReferenceContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(188);
      match(TromboneParser::IDENTIFIER);
      break;
    }

    default:
      break;
    }
    _ctx->stop = _input->LT(-1);
    setState(202);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 14, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        if (!_parseListeners.empty())
          triggerExitRuleEvent();
        previousContext = _localctx;
        setState(200);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 13, _ctx)) {
        case 1: {
          auto newContext = _tracker.createInstance<MulDivContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(191);

          if (!(precpred(_ctx, 11))) throw FailedPredicateException(this, "precpred(_ctx, 11)");
          setState(192);
          antlrcpp::downCast<MulDivContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == TromboneParser::T__18

          || _la == TromboneParser::T__19)) {
            antlrcpp::downCast<MulDivContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(193);
          expr(12);
          break;
        }

        case 2: {
          auto newContext = _tracker.createInstance<AddSubContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(194);

          if (!(precpred(_ctx, 10))) throw FailedPredicateException(this, "precpred(_ctx, 10)");
          setState(195);
          antlrcpp::downCast<AddSubContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == TromboneParser::T__20

          || _la == TromboneParser::T__21)) {
            antlrcpp::downCast<AddSubContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(196);
          expr(11);
          break;
        }

        case 3: {
          auto newContext = _tracker.createInstance<CompareContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(197);

          if (!(precpred(_ctx, 9))) throw FailedPredicateException(this, "precpred(_ctx, 9)");
          setState(198);
          antlrcpp::downCast<CompareContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!((((_la & ~ 0x3fULL) == 0) &&
            ((1ULL << _la) & 528482304) != 0))) {
            antlrcpp::downCast<CompareContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(199);
          expr(10);
          break;
        }

        default:
          break;
        } 
      }
      setState(204);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 14, _ctx);
    }
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }
  return _localctx;
}

bool TromboneParser::sempred(RuleContext *context, size_t ruleIndex, size_t predicateIndex) {
  switch (ruleIndex) {
    case 16: return exprSempred(antlrcpp::downCast<ExprContext *>(context), predicateIndex);

  default:
    break;
  }
  return true;
}

bool TromboneParser::exprSempred(ExprContext *_localctx, size_t predicateIndex) {
  switch (predicateIndex) {
    case 0: return precpred(_ctx, 11);
    case 1: return precpred(_ctx, 10);
    case 2: return precpred(_ctx, 9);

  default:
    break;
  }
  return true;
}

void TromboneParser::initialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  tromboneParserInitialize();
#else
  ::antlr4::internal::call_once(tromboneParserOnceFlag, tromboneParserInitialize);
#endif
}
