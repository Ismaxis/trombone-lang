
// Generated from ../../antlr/Trombone.g4 by ANTLR 4.13.2


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
      "returnStmt", "printStmt", "whileStmt", "ifStmt", "funcCall", "argList", 
      "expr"
    },
    std::vector<std::string>{
      "", "'fn'", "'('", "')'", "','", "':'", "'->'", "'int'", "'['", "']'", 
      "'{'", "'}'", "';'", "'let'", "'='", "'return'", "'print'", "'while'", 
      "'if'", "'else'", "'*'", "'/'", "'+'", "'-'", "'<'", "'>'", "'<='", 
      "'>='", "'=='", "'!='", "'read'", "'array'"
    },
    std::vector<std::string>{
      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", 
      "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "IDENTIFIER", 
      "NUMBER", "WS", "COMMENT"
    }
  );
  static const int32_t serializedATNSegment[] = {
  	4,1,35,213,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,6,2,
  	7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,13,2,14,7,
  	14,2,15,7,15,2,16,7,16,2,17,7,17,1,0,4,0,38,8,0,11,0,12,0,39,1,0,1,0,
  	1,1,1,1,1,1,1,1,3,1,48,8,1,1,1,1,1,3,1,52,8,1,1,1,1,1,1,2,1,2,1,2,5,2,
  	59,8,2,10,2,12,2,62,9,2,1,3,1,3,1,3,1,3,1,4,1,4,1,4,1,5,1,5,1,5,1,5,3,
  	5,75,8,5,1,6,1,6,5,6,79,8,6,10,6,12,6,82,9,6,1,6,1,6,1,7,1,7,1,7,1,7,
  	1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,3,7,106,
  	8,7,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,9,1,9,1,9,1,9,1,10,1,10,1,10,1,10,1,
  	10,1,10,1,10,1,11,1,11,1,11,1,12,1,12,1,12,1,12,1,12,1,13,1,13,1,13,1,
  	13,1,14,1,14,1,14,1,14,1,14,1,14,1,14,1,14,5,14,146,8,14,10,14,12,14,
  	149,9,14,1,14,1,14,3,14,153,8,14,1,15,1,15,1,15,3,15,158,8,15,1,15,1,
  	15,1,16,1,16,1,16,5,16,165,8,16,10,16,12,16,168,9,16,1,17,1,17,1,17,1,
  	17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,3,17,185,8,
  	17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,3,17,197,8,17,1,
  	17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,1,17,5,17,208,8,17,10,17,12,17,
  	211,9,17,1,17,0,1,34,18,0,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,
  	34,0,3,1,0,20,21,1,0,22,23,1,0,24,29,222,0,37,1,0,0,0,2,43,1,0,0,0,4,
  	55,1,0,0,0,6,63,1,0,0,0,8,67,1,0,0,0,10,74,1,0,0,0,12,76,1,0,0,0,14,105,
  	1,0,0,0,16,107,1,0,0,0,18,114,1,0,0,0,20,118,1,0,0,0,22,125,1,0,0,0,24,
  	128,1,0,0,0,26,133,1,0,0,0,28,137,1,0,0,0,30,154,1,0,0,0,32,161,1,0,0,
  	0,34,196,1,0,0,0,36,38,3,2,1,0,37,36,1,0,0,0,38,39,1,0,0,0,39,37,1,0,
  	0,0,39,40,1,0,0,0,40,41,1,0,0,0,41,42,5,0,0,1,42,1,1,0,0,0,43,44,5,1,
  	0,0,44,45,5,32,0,0,45,47,5,2,0,0,46,48,3,4,2,0,47,46,1,0,0,0,47,48,1,
  	0,0,0,48,49,1,0,0,0,49,51,5,3,0,0,50,52,3,8,4,0,51,50,1,0,0,0,51,52,1,
  	0,0,0,52,53,1,0,0,0,53,54,3,12,6,0,54,3,1,0,0,0,55,60,3,6,3,0,56,57,5,
  	4,0,0,57,59,3,6,3,0,58,56,1,0,0,0,59,62,1,0,0,0,60,58,1,0,0,0,60,61,1,
  	0,0,0,61,5,1,0,0,0,62,60,1,0,0,0,63,64,5,32,0,0,64,65,5,5,0,0,65,66,3,
  	10,5,0,66,7,1,0,0,0,67,68,5,6,0,0,68,69,3,10,5,0,69,9,1,0,0,0,70,75,5,
  	7,0,0,71,72,5,8,0,0,72,73,5,7,0,0,73,75,5,9,0,0,74,70,1,0,0,0,74,71,1,
  	0,0,0,75,11,1,0,0,0,76,80,5,10,0,0,77,79,3,14,7,0,78,77,1,0,0,0,79,82,
  	1,0,0,0,80,78,1,0,0,0,80,81,1,0,0,0,81,83,1,0,0,0,82,80,1,0,0,0,83,84,
  	5,11,0,0,84,13,1,0,0,0,85,86,3,16,8,0,86,87,5,12,0,0,87,106,1,0,0,0,88,
  	89,3,18,9,0,89,90,5,12,0,0,90,106,1,0,0,0,91,92,3,20,10,0,92,93,5,12,
  	0,0,93,106,1,0,0,0,94,95,3,30,15,0,95,96,5,12,0,0,96,106,1,0,0,0,97,98,
  	3,22,11,0,98,99,5,12,0,0,99,106,1,0,0,0,100,101,3,24,12,0,101,102,5,12,
  	0,0,102,106,1,0,0,0,103,106,3,26,13,0,104,106,3,28,14,0,105,85,1,0,0,
  	0,105,88,1,0,0,0,105,91,1,0,0,0,105,94,1,0,0,0,105,97,1,0,0,0,105,100,
  	1,0,0,0,105,103,1,0,0,0,105,104,1,0,0,0,106,15,1,0,0,0,107,108,5,13,0,
  	0,108,109,5,32,0,0,109,110,5,5,0,0,110,111,3,10,5,0,111,112,5,14,0,0,
  	112,113,3,34,17,0,113,17,1,0,0,0,114,115,5,32,0,0,115,116,5,14,0,0,116,
  	117,3,34,17,0,117,19,1,0,0,0,118,119,5,32,0,0,119,120,5,8,0,0,120,121,
  	3,34,17,0,121,122,5,9,0,0,122,123,5,14,0,0,123,124,3,34,17,0,124,21,1,
  	0,0,0,125,126,5,15,0,0,126,127,3,34,17,0,127,23,1,0,0,0,128,129,5,16,
  	0,0,129,130,5,2,0,0,130,131,3,34,17,0,131,132,5,3,0,0,132,25,1,0,0,0,
  	133,134,5,17,0,0,134,135,3,34,17,0,135,136,3,12,6,0,136,27,1,0,0,0,137,
  	138,5,18,0,0,138,139,3,34,17,0,139,147,3,12,6,0,140,141,5,19,0,0,141,
  	142,5,18,0,0,142,143,3,34,17,0,143,144,3,12,6,0,144,146,1,0,0,0,145,140,
  	1,0,0,0,146,149,1,0,0,0,147,145,1,0,0,0,147,148,1,0,0,0,148,152,1,0,0,
  	0,149,147,1,0,0,0,150,151,5,19,0,0,151,153,3,12,6,0,152,150,1,0,0,0,152,
  	153,1,0,0,0,153,29,1,0,0,0,154,155,5,32,0,0,155,157,5,2,0,0,156,158,3,
  	32,16,0,157,156,1,0,0,0,157,158,1,0,0,0,158,159,1,0,0,0,159,160,5,3,0,
  	0,160,31,1,0,0,0,161,166,3,34,17,0,162,163,5,4,0,0,163,165,3,34,17,0,
  	164,162,1,0,0,0,165,168,1,0,0,0,166,164,1,0,0,0,166,167,1,0,0,0,167,33,
  	1,0,0,0,168,166,1,0,0,0,169,170,6,17,-1,0,170,171,5,32,0,0,171,172,5,
  	8,0,0,172,173,3,34,17,0,173,174,5,9,0,0,174,197,1,0,0,0,175,197,3,30,
  	15,0,176,177,5,30,0,0,177,178,5,2,0,0,178,197,5,3,0,0,179,180,5,31,0,
  	0,180,181,5,2,0,0,181,184,3,34,17,0,182,183,5,4,0,0,183,185,3,34,17,0,
  	184,182,1,0,0,0,184,185,1,0,0,0,185,186,1,0,0,0,186,187,5,3,0,0,187,197,
  	1,0,0,0,188,189,5,2,0,0,189,190,3,34,17,0,190,191,5,3,0,0,191,197,1,0,
  	0,0,192,193,5,23,0,0,193,197,3,34,17,3,194,197,5,33,0,0,195,197,5,32,
  	0,0,196,169,1,0,0,0,196,175,1,0,0,0,196,176,1,0,0,0,196,179,1,0,0,0,196,
  	188,1,0,0,0,196,192,1,0,0,0,196,194,1,0,0,0,196,195,1,0,0,0,197,209,1,
  	0,0,0,198,199,10,11,0,0,199,200,7,0,0,0,200,208,3,34,17,12,201,202,10,
  	10,0,0,202,203,7,1,0,0,203,208,3,34,17,11,204,205,10,9,0,0,205,206,7,
  	2,0,0,206,208,3,34,17,10,207,198,1,0,0,0,207,201,1,0,0,0,207,204,1,0,
  	0,0,208,211,1,0,0,0,209,207,1,0,0,0,209,210,1,0,0,0,210,35,1,0,0,0,211,
  	209,1,0,0,0,15,39,47,51,60,74,80,105,147,152,157,166,184,196,207,209
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
    setState(37); 
    _errHandler->sync(this);
    _la = _input->LA(1);
    do {
      setState(36);
      functionDecl();
      setState(39); 
      _errHandler->sync(this);
      _la = _input->LA(1);
    } while (_la == TromboneParser::T__0);
    setState(41);
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
    setState(43);
    match(TromboneParser::T__0);
    setState(44);
    match(TromboneParser::IDENTIFIER);
    setState(45);
    match(TromboneParser::T__1);
    setState(47);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::IDENTIFIER) {
      setState(46);
      paramList();
    }
    setState(49);
    match(TromboneParser::T__2);
    setState(51);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__5) {
      setState(50);
      returnType();
    }
    setState(53);
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
    setState(55);
    param();
    setState(60);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(56);
      match(TromboneParser::T__3);
      setState(57);
      param();
      setState(62);
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
    setState(63);
    match(TromboneParser::IDENTIFIER);
    setState(64);
    match(TromboneParser::T__4);
    setState(65);
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
    setState(67);
    match(TromboneParser::T__5);
    setState(68);
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
    setState(74);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case TromboneParser::T__6: {
        enterOuterAlt(_localctx, 1);
        setState(70);
        match(TromboneParser::T__6);
        break;
      }

      case TromboneParser::T__7: {
        enterOuterAlt(_localctx, 2);
        setState(71);
        match(TromboneParser::T__7);
        setState(72);
        match(TromboneParser::T__6);
        setState(73);
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
    setState(76);
    match(TromboneParser::T__9);
    setState(80);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 4295467008) != 0)) {
      setState(77);
      statement();
      setState(82);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
    setState(83);
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

TromboneParser::PrintStmtContext* TromboneParser::StatementContext::printStmt() {
  return getRuleContext<TromboneParser::PrintStmtContext>(0);
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
    setState(105);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(85);
      varDecl();
      setState(86);
      match(TromboneParser::T__11);
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(88);
      assignment();
      setState(89);
      match(TromboneParser::T__11);
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(91);
      arrayAssignment();
      setState(92);
      match(TromboneParser::T__11);
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(94);
      funcCall();
      setState(95);
      match(TromboneParser::T__11);
      break;
    }

    case 5: {
      enterOuterAlt(_localctx, 5);
      setState(97);
      returnStmt();
      setState(98);
      match(TromboneParser::T__11);
      break;
    }

    case 6: {
      enterOuterAlt(_localctx, 6);
      setState(100);
      printStmt();
      setState(101);
      match(TromboneParser::T__11);
      break;
    }

    case 7: {
      enterOuterAlt(_localctx, 7);
      setState(103);
      whileStmt();
      break;
    }

    case 8: {
      enterOuterAlt(_localctx, 8);
      setState(104);
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
    setState(107);
    match(TromboneParser::T__12);
    setState(108);
    match(TromboneParser::IDENTIFIER);
    setState(109);
    match(TromboneParser::T__4);
    setState(110);
    type();
    setState(111);
    match(TromboneParser::T__13);
    setState(112);
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
    setState(114);
    match(TromboneParser::IDENTIFIER);
    setState(115);
    match(TromboneParser::T__13);
    setState(116);
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
    setState(118);
    match(TromboneParser::IDENTIFIER);
    setState(119);
    match(TromboneParser::T__7);
    setState(120);
    expr(0);
    setState(121);
    match(TromboneParser::T__8);
    setState(122);
    match(TromboneParser::T__13);
    setState(123);
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
    setState(125);
    match(TromboneParser::T__14);
    setState(126);
    expr(0);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- PrintStmtContext ------------------------------------------------------------------

TromboneParser::PrintStmtContext::PrintStmtContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::ExprContext* TromboneParser::PrintStmtContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}


size_t TromboneParser::PrintStmtContext::getRuleIndex() const {
  return TromboneParser::RulePrintStmt;
}


std::any TromboneParser::PrintStmtContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitPrintStmt(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::PrintStmtContext* TromboneParser::printStmt() {
  PrintStmtContext *_localctx = _tracker.createInstance<PrintStmtContext>(_ctx, getState());
  enterRule(_localctx, 24, TromboneParser::RulePrintStmt);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(128);
    match(TromboneParser::T__15);
    setState(129);
    match(TromboneParser::T__1);
    setState(130);
    expr(0);
    setState(131);
    match(TromboneParser::T__2);
   
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
  enterRule(_localctx, 26, TromboneParser::RuleWhileStmt);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(133);
    match(TromboneParser::T__16);
    setState(134);
    expr(0);
    setState(135);
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
  enterRule(_localctx, 28, TromboneParser::RuleIfStmt);
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
    setState(137);
    match(TromboneParser::T__17);
    setState(138);
    expr(0);
    setState(139);
    block();
    setState(147);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 7, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(140);
        match(TromboneParser::T__18);
        setState(141);
        match(TromboneParser::T__17);
        setState(142);
        expr(0);
        setState(143);
        block(); 
      }
      setState(149);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 7, _ctx);
    }
    setState(152);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__18) {
      setState(150);
      match(TromboneParser::T__18);
      setState(151);
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
  enterRule(_localctx, 30, TromboneParser::RuleFuncCall);
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
    setState(154);
    match(TromboneParser::IDENTIFIER);
    setState(155);
    match(TromboneParser::T__1);
    setState(157);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 16114515972) != 0)) {
      setState(156);
      argList();
    }
    setState(159);
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
  enterRule(_localctx, 32, TromboneParser::RuleArgList);
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
    setState(161);
    expr(0);
    setState(166);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(162);
      match(TromboneParser::T__3);
      setState(163);
      expr(0);
      setState(168);
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
//----------------- UnaryMinusContext ------------------------------------------------------------------

TromboneParser::ExprContext* TromboneParser::UnaryMinusContext::expr() {
  return getRuleContext<TromboneParser::ExprContext>(0);
}

TromboneParser::UnaryMinusContext::UnaryMinusContext(ExprContext *ctx) { copyFrom(ctx); }


std::any TromboneParser::UnaryMinusContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitUnaryMinus(this);
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
  size_t startState = 34;
  enterRecursionRule(_localctx, 34, TromboneParser::RuleExpr, precedence);

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
    setState(196);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 12, _ctx)) {
    case 1: {
      _localctx = _tracker.createInstance<ArrayAccessContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;

      setState(170);
      match(TromboneParser::IDENTIFIER);
      setState(171);
      match(TromboneParser::T__7);
      setState(172);
      expr(0);
      setState(173);
      match(TromboneParser::T__8);
      break;
    }

    case 2: {
      _localctx = _tracker.createInstance<FuncCallExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(175);
      funcCall();
      break;
    }

    case 3: {
      _localctx = _tracker.createInstance<ReadExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(176);
      match(TromboneParser::T__29);
      setState(177);
      match(TromboneParser::T__1);
      setState(178);
      match(TromboneParser::T__2);
      break;
    }

    case 4: {
      _localctx = _tracker.createInstance<ArrayCreateContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(179);
      match(TromboneParser::T__30);
      setState(180);
      match(TromboneParser::T__1);
      setState(181);
      expr(0);
      setState(184);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == TromboneParser::T__3) {
        setState(182);
        match(TromboneParser::T__3);
        setState(183);
        expr(0);
      }
      setState(186);
      match(TromboneParser::T__2);
      break;
    }

    case 5: {
      _localctx = _tracker.createInstance<ParensContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(188);
      match(TromboneParser::T__1);
      setState(189);
      expr(0);
      setState(190);
      match(TromboneParser::T__2);
      break;
    }

    case 6: {
      _localctx = _tracker.createInstance<UnaryMinusContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(192);
      match(TromboneParser::T__22);
      setState(193);
      expr(3);
      break;
    }

    case 7: {
      _localctx = _tracker.createInstance<IntLiteralContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(194);
      match(TromboneParser::NUMBER);
      break;
    }

    case 8: {
      _localctx = _tracker.createInstance<VarReferenceContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(195);
      match(TromboneParser::IDENTIFIER);
      break;
    }

    default:
      break;
    }
    _ctx->stop = _input->LT(-1);
    setState(209);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 14, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        if (!_parseListeners.empty())
          triggerExitRuleEvent();
        previousContext = _localctx;
        setState(207);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 13, _ctx)) {
        case 1: {
          auto newContext = _tracker.createInstance<MulDivContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(198);

          if (!(precpred(_ctx, 11))) throw FailedPredicateException(this, "precpred(_ctx, 11)");
          setState(199);
          antlrcpp::downCast<MulDivContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == TromboneParser::T__19

          || _la == TromboneParser::T__20)) {
            antlrcpp::downCast<MulDivContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(200);
          expr(12);
          break;
        }

        case 2: {
          auto newContext = _tracker.createInstance<AddSubContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(201);

          if (!(precpred(_ctx, 10))) throw FailedPredicateException(this, "precpred(_ctx, 10)");
          setState(202);
          antlrcpp::downCast<AddSubContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == TromboneParser::T__21

          || _la == TromboneParser::T__22)) {
            antlrcpp::downCast<AddSubContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(203);
          expr(11);
          break;
        }

        case 3: {
          auto newContext = _tracker.createInstance<CompareContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(204);

          if (!(precpred(_ctx, 9))) throw FailedPredicateException(this, "precpred(_ctx, 9)");
          setState(205);
          antlrcpp::downCast<CompareContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!((((_la & ~ 0x3fULL) == 0) &&
            ((1ULL << _la) & 1056964608) != 0))) {
            antlrcpp::downCast<CompareContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(206);
          expr(10);
          break;
        }

        default:
          break;
        } 
      }
      setState(211);
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
    case 17: return exprSempred(antlrcpp::downCast<ExprContext *>(context), predicateIndex);

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
