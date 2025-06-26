
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
      "returnStmt", "printStmt", "whileStmt", "funcCallStmt", "ifStmt", 
      "funcCall", "argList", "expr"
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
  	4,1,35,218,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,6,2,
  	7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,13,2,14,7,
  	14,2,15,7,15,2,16,7,16,2,17,7,17,2,18,7,18,1,0,4,0,40,8,0,11,0,12,0,41,
  	1,0,1,0,1,1,1,1,1,1,1,1,3,1,50,8,1,1,1,1,1,3,1,54,8,1,1,1,1,1,1,2,1,2,
  	1,2,5,2,61,8,2,10,2,12,2,64,9,2,1,3,1,3,1,3,1,3,1,4,1,4,1,4,1,5,1,5,1,
  	5,1,5,3,5,77,8,5,1,6,1,6,5,6,81,8,6,10,6,12,6,84,9,6,1,6,1,6,1,7,1,7,
  	1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,1,
  	7,3,7,108,8,7,1,8,1,8,1,8,1,8,1,8,1,8,1,8,1,9,1,9,1,9,1,9,1,10,1,10,1,
  	10,1,10,1,10,1,10,1,10,1,11,1,11,3,11,130,8,11,1,12,1,12,1,12,1,12,1,
  	12,1,13,1,13,1,13,1,13,1,14,1,14,1,15,1,15,1,15,1,15,1,15,1,15,1,15,1,
  	15,5,15,151,8,15,10,15,12,15,154,9,15,1,15,1,15,3,15,158,8,15,1,16,1,
  	16,1,16,3,16,163,8,16,1,16,1,16,1,17,1,17,1,17,5,17,170,8,17,10,17,12,
  	17,173,9,17,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,
  	18,1,18,1,18,1,18,3,18,190,8,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,
  	18,1,18,1,18,3,18,202,8,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,18,1,
  	18,5,18,213,8,18,10,18,12,18,216,9,18,1,18,0,1,36,19,0,2,4,6,8,10,12,
  	14,16,18,20,22,24,26,28,30,32,34,36,0,3,1,0,20,21,1,0,22,23,1,0,24,29,
  	227,0,39,1,0,0,0,2,45,1,0,0,0,4,57,1,0,0,0,6,65,1,0,0,0,8,69,1,0,0,0,
  	10,76,1,0,0,0,12,78,1,0,0,0,14,107,1,0,0,0,16,109,1,0,0,0,18,116,1,0,
  	0,0,20,120,1,0,0,0,22,127,1,0,0,0,24,131,1,0,0,0,26,136,1,0,0,0,28,140,
  	1,0,0,0,30,142,1,0,0,0,32,159,1,0,0,0,34,166,1,0,0,0,36,201,1,0,0,0,38,
  	40,3,2,1,0,39,38,1,0,0,0,40,41,1,0,0,0,41,39,1,0,0,0,41,42,1,0,0,0,42,
  	43,1,0,0,0,43,44,5,0,0,1,44,1,1,0,0,0,45,46,5,1,0,0,46,47,5,32,0,0,47,
  	49,5,2,0,0,48,50,3,4,2,0,49,48,1,0,0,0,49,50,1,0,0,0,50,51,1,0,0,0,51,
  	53,5,3,0,0,52,54,3,8,4,0,53,52,1,0,0,0,53,54,1,0,0,0,54,55,1,0,0,0,55,
  	56,3,12,6,0,56,3,1,0,0,0,57,62,3,6,3,0,58,59,5,4,0,0,59,61,3,6,3,0,60,
  	58,1,0,0,0,61,64,1,0,0,0,62,60,1,0,0,0,62,63,1,0,0,0,63,5,1,0,0,0,64,
  	62,1,0,0,0,65,66,5,32,0,0,66,67,5,5,0,0,67,68,3,10,5,0,68,7,1,0,0,0,69,
  	70,5,6,0,0,70,71,3,10,5,0,71,9,1,0,0,0,72,77,5,7,0,0,73,74,5,8,0,0,74,
  	75,5,7,0,0,75,77,5,9,0,0,76,72,1,0,0,0,76,73,1,0,0,0,77,11,1,0,0,0,78,
  	82,5,10,0,0,79,81,3,14,7,0,80,79,1,0,0,0,81,84,1,0,0,0,82,80,1,0,0,0,
  	82,83,1,0,0,0,83,85,1,0,0,0,84,82,1,0,0,0,85,86,5,11,0,0,86,13,1,0,0,
  	0,87,88,3,16,8,0,88,89,5,12,0,0,89,108,1,0,0,0,90,91,3,18,9,0,91,92,5,
  	12,0,0,92,108,1,0,0,0,93,94,3,20,10,0,94,95,5,12,0,0,95,108,1,0,0,0,96,
  	97,3,28,14,0,97,98,5,12,0,0,98,108,1,0,0,0,99,100,3,22,11,0,100,101,5,
  	12,0,0,101,108,1,0,0,0,102,103,3,24,12,0,103,104,5,12,0,0,104,108,1,0,
  	0,0,105,108,3,26,13,0,106,108,3,30,15,0,107,87,1,0,0,0,107,90,1,0,0,0,
  	107,93,1,0,0,0,107,96,1,0,0,0,107,99,1,0,0,0,107,102,1,0,0,0,107,105,
  	1,0,0,0,107,106,1,0,0,0,108,15,1,0,0,0,109,110,5,13,0,0,110,111,5,32,
  	0,0,111,112,5,5,0,0,112,113,3,10,5,0,113,114,5,14,0,0,114,115,3,36,18,
  	0,115,17,1,0,0,0,116,117,5,32,0,0,117,118,5,14,0,0,118,119,3,36,18,0,
  	119,19,1,0,0,0,120,121,5,32,0,0,121,122,5,8,0,0,122,123,3,36,18,0,123,
  	124,5,9,0,0,124,125,5,14,0,0,125,126,3,36,18,0,126,21,1,0,0,0,127,129,
  	5,15,0,0,128,130,3,36,18,0,129,128,1,0,0,0,129,130,1,0,0,0,130,23,1,0,
  	0,0,131,132,5,16,0,0,132,133,5,2,0,0,133,134,3,36,18,0,134,135,5,3,0,
  	0,135,25,1,0,0,0,136,137,5,17,0,0,137,138,3,36,18,0,138,139,3,12,6,0,
  	139,27,1,0,0,0,140,141,3,32,16,0,141,29,1,0,0,0,142,143,5,18,0,0,143,
  	144,3,36,18,0,144,152,3,12,6,0,145,146,5,19,0,0,146,147,5,18,0,0,147,
  	148,3,36,18,0,148,149,3,12,6,0,149,151,1,0,0,0,150,145,1,0,0,0,151,154,
  	1,0,0,0,152,150,1,0,0,0,152,153,1,0,0,0,153,157,1,0,0,0,154,152,1,0,0,
  	0,155,156,5,19,0,0,156,158,3,12,6,0,157,155,1,0,0,0,157,158,1,0,0,0,158,
  	31,1,0,0,0,159,160,5,32,0,0,160,162,5,2,0,0,161,163,3,34,17,0,162,161,
  	1,0,0,0,162,163,1,0,0,0,163,164,1,0,0,0,164,165,5,3,0,0,165,33,1,0,0,
  	0,166,171,3,36,18,0,167,168,5,4,0,0,168,170,3,36,18,0,169,167,1,0,0,0,
  	170,173,1,0,0,0,171,169,1,0,0,0,171,172,1,0,0,0,172,35,1,0,0,0,173,171,
  	1,0,0,0,174,175,6,18,-1,0,175,176,5,32,0,0,176,177,5,8,0,0,177,178,3,
  	36,18,0,178,179,5,9,0,0,179,202,1,0,0,0,180,202,3,32,16,0,181,182,5,30,
  	0,0,182,183,5,2,0,0,183,202,5,3,0,0,184,185,5,31,0,0,185,186,5,2,0,0,
  	186,189,3,36,18,0,187,188,5,4,0,0,188,190,3,36,18,0,189,187,1,0,0,0,189,
  	190,1,0,0,0,190,191,1,0,0,0,191,192,5,3,0,0,192,202,1,0,0,0,193,194,5,
  	2,0,0,194,195,3,36,18,0,195,196,5,3,0,0,196,202,1,0,0,0,197,198,5,23,
  	0,0,198,202,3,36,18,3,199,202,5,33,0,0,200,202,5,32,0,0,201,174,1,0,0,
  	0,201,180,1,0,0,0,201,181,1,0,0,0,201,184,1,0,0,0,201,193,1,0,0,0,201,
  	197,1,0,0,0,201,199,1,0,0,0,201,200,1,0,0,0,202,214,1,0,0,0,203,204,10,
  	11,0,0,204,205,7,0,0,0,205,213,3,36,18,12,206,207,10,10,0,0,207,208,7,
  	1,0,0,208,213,3,36,18,11,209,210,10,9,0,0,210,211,7,2,0,0,211,213,3,36,
  	18,10,212,203,1,0,0,0,212,206,1,0,0,0,212,209,1,0,0,0,213,216,1,0,0,0,
  	214,212,1,0,0,0,214,215,1,0,0,0,215,37,1,0,0,0,216,214,1,0,0,0,16,41,
  	49,53,62,76,82,107,129,152,157,162,171,189,201,212,214
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
    setState(39); 
    _errHandler->sync(this);
    _la = _input->LA(1);
    do {
      setState(38);
      functionDecl();
      setState(41); 
      _errHandler->sync(this);
      _la = _input->LA(1);
    } while (_la == TromboneParser::T__0);
    setState(43);
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
    setState(45);
    match(TromboneParser::T__0);
    setState(46);
    match(TromboneParser::IDENTIFIER);
    setState(47);
    match(TromboneParser::T__1);
    setState(49);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::IDENTIFIER) {
      setState(48);
      paramList();
    }
    setState(51);
    match(TromboneParser::T__2);
    setState(53);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__5) {
      setState(52);
      returnType();
    }
    setState(55);
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
    setState(57);
    param();
    setState(62);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(58);
      match(TromboneParser::T__3);
      setState(59);
      param();
      setState(64);
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
    setState(65);
    match(TromboneParser::IDENTIFIER);
    setState(66);
    match(TromboneParser::T__4);
    setState(67);
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
    setState(69);
    match(TromboneParser::T__5);
    setState(70);
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
    setState(76);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case TromboneParser::T__6: {
        enterOuterAlt(_localctx, 1);
        setState(72);
        match(TromboneParser::T__6);
        break;
      }

      case TromboneParser::T__7: {
        enterOuterAlt(_localctx, 2);
        setState(73);
        match(TromboneParser::T__7);
        setState(74);
        match(TromboneParser::T__6);
        setState(75);
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
    setState(78);
    match(TromboneParser::T__9);
    setState(82);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 4295467008) != 0)) {
      setState(79);
      statement();
      setState(84);
      _errHandler->sync(this);
      _la = _input->LA(1);
    }
    setState(85);
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

TromboneParser::FuncCallStmtContext* TromboneParser::StatementContext::funcCallStmt() {
  return getRuleContext<TromboneParser::FuncCallStmtContext>(0);
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
    setState(107);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(87);
      varDecl();
      setState(88);
      match(TromboneParser::T__11);
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(90);
      assignment();
      setState(91);
      match(TromboneParser::T__11);
      break;
    }

    case 3: {
      enterOuterAlt(_localctx, 3);
      setState(93);
      arrayAssignment();
      setState(94);
      match(TromboneParser::T__11);
      break;
    }

    case 4: {
      enterOuterAlt(_localctx, 4);
      setState(96);
      funcCallStmt();
      setState(97);
      match(TromboneParser::T__11);
      break;
    }

    case 5: {
      enterOuterAlt(_localctx, 5);
      setState(99);
      returnStmt();
      setState(100);
      match(TromboneParser::T__11);
      break;
    }

    case 6: {
      enterOuterAlt(_localctx, 6);
      setState(102);
      printStmt();
      setState(103);
      match(TromboneParser::T__11);
      break;
    }

    case 7: {
      enterOuterAlt(_localctx, 7);
      setState(105);
      whileStmt();
      break;
    }

    case 8: {
      enterOuterAlt(_localctx, 8);
      setState(106);
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
    setState(109);
    match(TromboneParser::T__12);
    setState(110);
    match(TromboneParser::IDENTIFIER);
    setState(111);
    match(TromboneParser::T__4);
    setState(112);
    type();
    setState(113);
    match(TromboneParser::T__13);
    setState(114);
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
    setState(116);
    match(TromboneParser::IDENTIFIER);
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
    setState(120);
    match(TromboneParser::IDENTIFIER);
    setState(121);
    match(TromboneParser::T__7);
    setState(122);
    expr(0);
    setState(123);
    match(TromboneParser::T__8);
    setState(124);
    match(TromboneParser::T__13);
    setState(125);
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
    setState(127);
    match(TromboneParser::T__14);
    setState(129);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 16114515972) != 0)) {
      setState(128);
      expr(0);
    }
   
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
    setState(131);
    match(TromboneParser::T__15);
    setState(132);
    match(TromboneParser::T__1);
    setState(133);
    expr(0);
    setState(134);
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
    setState(136);
    match(TromboneParser::T__16);
    setState(137);
    expr(0);
    setState(138);
    block();
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- FuncCallStmtContext ------------------------------------------------------------------

TromboneParser::FuncCallStmtContext::FuncCallStmtContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

TromboneParser::FuncCallContext* TromboneParser::FuncCallStmtContext::funcCall() {
  return getRuleContext<TromboneParser::FuncCallContext>(0);
}


size_t TromboneParser::FuncCallStmtContext::getRuleIndex() const {
  return TromboneParser::RuleFuncCallStmt;
}


std::any TromboneParser::FuncCallStmtContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<TromboneVisitor*>(visitor))
    return parserVisitor->visitFuncCallStmt(this);
  else
    return visitor->visitChildren(this);
}

TromboneParser::FuncCallStmtContext* TromboneParser::funcCallStmt() {
  FuncCallStmtContext *_localctx = _tracker.createInstance<FuncCallStmtContext>(_ctx, getState());
  enterRule(_localctx, 28, TromboneParser::RuleFuncCallStmt);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(140);
    funcCall();
   
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
  enterRule(_localctx, 30, TromboneParser::RuleIfStmt);
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
    setState(142);
    match(TromboneParser::T__17);
    setState(143);
    expr(0);
    setState(144);
    block();
    setState(152);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 8, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        setState(145);
        match(TromboneParser::T__18);
        setState(146);
        match(TromboneParser::T__17);
        setState(147);
        expr(0);
        setState(148);
        block(); 
      }
      setState(154);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 8, _ctx);
    }
    setState(157);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if (_la == TromboneParser::T__18) {
      setState(155);
      match(TromboneParser::T__18);
      setState(156);
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
  enterRule(_localctx, 32, TromboneParser::RuleFuncCall);
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
    setState(159);
    match(TromboneParser::IDENTIFIER);
    setState(160);
    match(TromboneParser::T__1);
    setState(162);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 16114515972) != 0)) {
      setState(161);
      argList();
    }
    setState(164);
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
  enterRule(_localctx, 34, TromboneParser::RuleArgList);
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
    setState(166);
    expr(0);
    setState(171);
    _errHandler->sync(this);
    _la = _input->LA(1);
    while (_la == TromboneParser::T__3) {
      setState(167);
      match(TromboneParser::T__3);
      setState(168);
      expr(0);
      setState(173);
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
  size_t startState = 36;
  enterRecursionRule(_localctx, 36, TromboneParser::RuleExpr, precedence);

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
    setState(201);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 13, _ctx)) {
    case 1: {
      _localctx = _tracker.createInstance<ArrayAccessContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;

      setState(175);
      match(TromboneParser::IDENTIFIER);
      setState(176);
      match(TromboneParser::T__7);
      setState(177);
      expr(0);
      setState(178);
      match(TromboneParser::T__8);
      break;
    }

    case 2: {
      _localctx = _tracker.createInstance<FuncCallExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(180);
      funcCall();
      break;
    }

    case 3: {
      _localctx = _tracker.createInstance<ReadExprContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(181);
      match(TromboneParser::T__29);
      setState(182);
      match(TromboneParser::T__1);
      setState(183);
      match(TromboneParser::T__2);
      break;
    }

    case 4: {
      _localctx = _tracker.createInstance<ArrayCreateContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(184);
      match(TromboneParser::T__30);
      setState(185);
      match(TromboneParser::T__1);
      setState(186);
      expr(0);
      setState(189);
      _errHandler->sync(this);

      _la = _input->LA(1);
      if (_la == TromboneParser::T__3) {
        setState(187);
        match(TromboneParser::T__3);
        setState(188);
        expr(0);
      }
      setState(191);
      match(TromboneParser::T__2);
      break;
    }

    case 5: {
      _localctx = _tracker.createInstance<ParensContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(193);
      match(TromboneParser::T__1);
      setState(194);
      expr(0);
      setState(195);
      match(TromboneParser::T__2);
      break;
    }

    case 6: {
      _localctx = _tracker.createInstance<UnaryMinusContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(197);
      match(TromboneParser::T__22);
      setState(198);
      expr(3);
      break;
    }

    case 7: {
      _localctx = _tracker.createInstance<IntLiteralContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(199);
      match(TromboneParser::NUMBER);
      break;
    }

    case 8: {
      _localctx = _tracker.createInstance<VarReferenceContext>(_localctx);
      _ctx = _localctx;
      previousContext = _localctx;
      setState(200);
      match(TromboneParser::IDENTIFIER);
      break;
    }

    default:
      break;
    }
    _ctx->stop = _input->LT(-1);
    setState(214);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 15, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        if (!_parseListeners.empty())
          triggerExitRuleEvent();
        previousContext = _localctx;
        setState(212);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 14, _ctx)) {
        case 1: {
          auto newContext = _tracker.createInstance<MulDivContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(203);

          if (!(precpred(_ctx, 11))) throw FailedPredicateException(this, "precpred(_ctx, 11)");
          setState(204);
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
          setState(205);
          expr(12);
          break;
        }

        case 2: {
          auto newContext = _tracker.createInstance<AddSubContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(206);

          if (!(precpred(_ctx, 10))) throw FailedPredicateException(this, "precpred(_ctx, 10)");
          setState(207);
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
          setState(208);
          expr(11);
          break;
        }

        case 3: {
          auto newContext = _tracker.createInstance<CompareContext>(_tracker.createInstance<ExprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleExpr);
          setState(209);

          if (!(precpred(_ctx, 9))) throw FailedPredicateException(this, "precpred(_ctx, 9)");
          setState(210);
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
          setState(211);
          expr(10);
          break;
        }

        default:
          break;
        } 
      }
      setState(216);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 15, _ctx);
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
    case 18: return exprSempred(antlrcpp::downCast<ExprContext *>(context), predicateIndex);

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
