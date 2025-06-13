
// Generated from ./antlr/Trombone.g4 by ANTLR 4.13.2


#include "TromboneLexer.h"


using namespace antlr4;



using namespace antlr4;

namespace {

struct TromboneLexerStaticData final {
  TromboneLexerStaticData(std::vector<std::string> ruleNames,
                          std::vector<std::string> channelNames,
                          std::vector<std::string> modeNames,
                          std::vector<std::string> literalNames,
                          std::vector<std::string> symbolicNames)
      : ruleNames(std::move(ruleNames)), channelNames(std::move(channelNames)),
        modeNames(std::move(modeNames)), literalNames(std::move(literalNames)),
        symbolicNames(std::move(symbolicNames)),
        vocabulary(this->literalNames, this->symbolicNames) {}

  TromboneLexerStaticData(const TromboneLexerStaticData&) = delete;
  TromboneLexerStaticData(TromboneLexerStaticData&&) = delete;
  TromboneLexerStaticData& operator=(const TromboneLexerStaticData&) = delete;
  TromboneLexerStaticData& operator=(TromboneLexerStaticData&&) = delete;

  std::vector<antlr4::dfa::DFA> decisionToDFA;
  antlr4::atn::PredictionContextCache sharedContextCache;
  const std::vector<std::string> ruleNames;
  const std::vector<std::string> channelNames;
  const std::vector<std::string> modeNames;
  const std::vector<std::string> literalNames;
  const std::vector<std::string> symbolicNames;
  const antlr4::dfa::Vocabulary vocabulary;
  antlr4::atn::SerializedATNView serializedATN;
  std::unique_ptr<antlr4::atn::ATN> atn;
};

::antlr4::internal::OnceFlag trombonelexerLexerOnceFlag;
#if ANTLR4_USE_THREAD_LOCAL_CACHE
static thread_local
#endif
std::unique_ptr<TromboneLexerStaticData> trombonelexerLexerStaticData = nullptr;

void trombonelexerLexerInitialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  if (trombonelexerLexerStaticData != nullptr) {
    return;
  }
#else
  assert(trombonelexerLexerStaticData == nullptr);
#endif
  auto staticData = std::make_unique<TromboneLexerStaticData>(
    std::vector<std::string>{
      "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
      "T__9", "T__10", "T__11", "T__12", "T__13", "T__14", "T__15", "T__16", 
      "T__17", "T__18", "T__19", "T__20", "T__21", "T__22", "T__23", "T__24", 
      "T__25", "T__26", "T__27", "T__28", "T__29", "T__30", "IDENTIFIER", 
      "NUMBER", "WS", "COMMENT"
    },
    std::vector<std::string>{
      "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
    },
    std::vector<std::string>{
      "DEFAULT_MODE"
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
  	4,0,35,197,6,-1,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,
  	6,2,7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,13,7,13,2,14,
  	7,14,2,15,7,15,2,16,7,16,2,17,7,17,2,18,7,18,2,19,7,19,2,20,7,20,2,21,
  	7,21,2,22,7,22,2,23,7,23,2,24,7,24,2,25,7,25,2,26,7,26,2,27,7,27,2,28,
  	7,28,2,29,7,29,2,30,7,30,2,31,7,31,2,32,7,32,2,33,7,33,2,34,7,34,1,0,
  	1,0,1,0,1,1,1,1,1,2,1,2,1,3,1,3,1,4,1,4,1,5,1,5,1,5,1,6,1,6,1,6,1,6,1,
  	7,1,7,1,8,1,8,1,9,1,9,1,10,1,10,1,11,1,11,1,12,1,12,1,12,1,12,1,13,1,
  	13,1,14,1,14,1,14,1,14,1,14,1,14,1,14,1,15,1,15,1,15,1,15,1,15,1,15,1,
  	16,1,16,1,16,1,17,1,17,1,17,1,17,1,17,1,18,1,18,1,19,1,19,1,20,1,20,1,
  	21,1,21,1,22,1,22,1,23,1,23,1,24,1,24,1,24,1,25,1,25,1,25,1,26,1,26,1,
  	26,1,27,1,27,1,27,1,28,1,28,1,28,1,28,1,28,1,29,1,29,1,29,1,29,1,29,1,
  	29,1,30,1,30,1,30,1,30,1,30,1,30,1,31,1,31,5,31,170,8,31,10,31,12,31,
  	173,9,31,1,32,4,32,176,8,32,11,32,12,32,177,1,33,4,33,181,8,33,11,33,
  	12,33,182,1,33,1,33,1,34,1,34,1,34,1,34,5,34,191,8,34,10,34,12,34,194,
  	9,34,1,34,1,34,0,0,35,1,1,3,2,5,3,7,4,9,5,11,6,13,7,15,8,17,9,19,10,21,
  	11,23,12,25,13,27,14,29,15,31,16,33,17,35,18,37,19,39,20,41,21,43,22,
  	45,23,47,24,49,25,51,26,53,27,55,28,57,29,59,30,61,31,63,32,65,33,67,
  	34,69,35,1,0,5,3,0,65,90,95,95,97,122,4,0,48,57,65,90,95,95,97,122,1,
  	0,48,57,3,0,9,10,13,13,32,32,2,0,10,10,13,13,200,0,1,1,0,0,0,0,3,1,0,
  	0,0,0,5,1,0,0,0,0,7,1,0,0,0,0,9,1,0,0,0,0,11,1,0,0,0,0,13,1,0,0,0,0,15,
  	1,0,0,0,0,17,1,0,0,0,0,19,1,0,0,0,0,21,1,0,0,0,0,23,1,0,0,0,0,25,1,0,
  	0,0,0,27,1,0,0,0,0,29,1,0,0,0,0,31,1,0,0,0,0,33,1,0,0,0,0,35,1,0,0,0,
  	0,37,1,0,0,0,0,39,1,0,0,0,0,41,1,0,0,0,0,43,1,0,0,0,0,45,1,0,0,0,0,47,
  	1,0,0,0,0,49,1,0,0,0,0,51,1,0,0,0,0,53,1,0,0,0,0,55,1,0,0,0,0,57,1,0,
  	0,0,0,59,1,0,0,0,0,61,1,0,0,0,0,63,1,0,0,0,0,65,1,0,0,0,0,67,1,0,0,0,
  	0,69,1,0,0,0,1,71,1,0,0,0,3,74,1,0,0,0,5,76,1,0,0,0,7,78,1,0,0,0,9,80,
  	1,0,0,0,11,82,1,0,0,0,13,85,1,0,0,0,15,89,1,0,0,0,17,91,1,0,0,0,19,93,
  	1,0,0,0,21,95,1,0,0,0,23,97,1,0,0,0,25,99,1,0,0,0,27,103,1,0,0,0,29,105,
  	1,0,0,0,31,112,1,0,0,0,33,118,1,0,0,0,35,121,1,0,0,0,37,126,1,0,0,0,39,
  	128,1,0,0,0,41,130,1,0,0,0,43,132,1,0,0,0,45,134,1,0,0,0,47,136,1,0,0,
  	0,49,138,1,0,0,0,51,141,1,0,0,0,53,144,1,0,0,0,55,147,1,0,0,0,57,150,
  	1,0,0,0,59,155,1,0,0,0,61,161,1,0,0,0,63,167,1,0,0,0,65,175,1,0,0,0,67,
  	180,1,0,0,0,69,186,1,0,0,0,71,72,5,102,0,0,72,73,5,110,0,0,73,2,1,0,0,
  	0,74,75,5,40,0,0,75,4,1,0,0,0,76,77,5,41,0,0,77,6,1,0,0,0,78,79,5,44,
  	0,0,79,8,1,0,0,0,80,81,5,58,0,0,81,10,1,0,0,0,82,83,5,45,0,0,83,84,5,
  	62,0,0,84,12,1,0,0,0,85,86,5,105,0,0,86,87,5,110,0,0,87,88,5,116,0,0,
  	88,14,1,0,0,0,89,90,5,91,0,0,90,16,1,0,0,0,91,92,5,93,0,0,92,18,1,0,0,
  	0,93,94,5,123,0,0,94,20,1,0,0,0,95,96,5,125,0,0,96,22,1,0,0,0,97,98,5,
  	59,0,0,98,24,1,0,0,0,99,100,5,108,0,0,100,101,5,101,0,0,101,102,5,116,
  	0,0,102,26,1,0,0,0,103,104,5,61,0,0,104,28,1,0,0,0,105,106,5,114,0,0,
  	106,107,5,101,0,0,107,108,5,116,0,0,108,109,5,117,0,0,109,110,5,114,0,
  	0,110,111,5,110,0,0,111,30,1,0,0,0,112,113,5,119,0,0,113,114,5,104,0,
  	0,114,115,5,105,0,0,115,116,5,108,0,0,116,117,5,101,0,0,117,32,1,0,0,
  	0,118,119,5,105,0,0,119,120,5,102,0,0,120,34,1,0,0,0,121,122,5,101,0,
  	0,122,123,5,108,0,0,123,124,5,115,0,0,124,125,5,101,0,0,125,36,1,0,0,
  	0,126,127,5,42,0,0,127,38,1,0,0,0,128,129,5,47,0,0,129,40,1,0,0,0,130,
  	131,5,43,0,0,131,42,1,0,0,0,132,133,5,45,0,0,133,44,1,0,0,0,134,135,5,
  	60,0,0,135,46,1,0,0,0,136,137,5,62,0,0,137,48,1,0,0,0,138,139,5,60,0,
  	0,139,140,5,61,0,0,140,50,1,0,0,0,141,142,5,62,0,0,142,143,5,61,0,0,143,
  	52,1,0,0,0,144,145,5,61,0,0,145,146,5,61,0,0,146,54,1,0,0,0,147,148,5,
  	33,0,0,148,149,5,61,0,0,149,56,1,0,0,0,150,151,5,114,0,0,151,152,5,101,
  	0,0,152,153,5,97,0,0,153,154,5,100,0,0,154,58,1,0,0,0,155,156,5,112,0,
  	0,156,157,5,114,0,0,157,158,5,105,0,0,158,159,5,110,0,0,159,160,5,116,
  	0,0,160,60,1,0,0,0,161,162,5,97,0,0,162,163,5,114,0,0,163,164,5,114,0,
  	0,164,165,5,97,0,0,165,166,5,121,0,0,166,62,1,0,0,0,167,171,7,0,0,0,168,
  	170,7,1,0,0,169,168,1,0,0,0,170,173,1,0,0,0,171,169,1,0,0,0,171,172,1,
  	0,0,0,172,64,1,0,0,0,173,171,1,0,0,0,174,176,7,2,0,0,175,174,1,0,0,0,
  	176,177,1,0,0,0,177,175,1,0,0,0,177,178,1,0,0,0,178,66,1,0,0,0,179,181,
  	7,3,0,0,180,179,1,0,0,0,181,182,1,0,0,0,182,180,1,0,0,0,182,183,1,0,0,
  	0,183,184,1,0,0,0,184,185,6,33,0,0,185,68,1,0,0,0,186,187,5,47,0,0,187,
  	188,5,47,0,0,188,192,1,0,0,0,189,191,8,4,0,0,190,189,1,0,0,0,191,194,
  	1,0,0,0,192,190,1,0,0,0,192,193,1,0,0,0,193,195,1,0,0,0,194,192,1,0,0,
  	0,195,196,6,34,0,0,196,70,1,0,0,0,5,0,171,177,182,192,1,6,0,0
  };
  staticData->serializedATN = antlr4::atn::SerializedATNView(serializedATNSegment, sizeof(serializedATNSegment) / sizeof(serializedATNSegment[0]));

  antlr4::atn::ATNDeserializer deserializer;
  staticData->atn = deserializer.deserialize(staticData->serializedATN);

  const size_t count = staticData->atn->getNumberOfDecisions();
  staticData->decisionToDFA.reserve(count);
  for (size_t i = 0; i < count; i++) { 
    staticData->decisionToDFA.emplace_back(staticData->atn->getDecisionState(i), i);
  }
  trombonelexerLexerStaticData = std::move(staticData);
}

}

TromboneLexer::TromboneLexer(CharStream *input) : Lexer(input) {
  TromboneLexer::initialize();
  _interpreter = new atn::LexerATNSimulator(this, *trombonelexerLexerStaticData->atn, trombonelexerLexerStaticData->decisionToDFA, trombonelexerLexerStaticData->sharedContextCache);
}

TromboneLexer::~TromboneLexer() {
  delete _interpreter;
}

std::string TromboneLexer::getGrammarFileName() const {
  return "Trombone.g4";
}

const std::vector<std::string>& TromboneLexer::getRuleNames() const {
  return trombonelexerLexerStaticData->ruleNames;
}

const std::vector<std::string>& TromboneLexer::getChannelNames() const {
  return trombonelexerLexerStaticData->channelNames;
}

const std::vector<std::string>& TromboneLexer::getModeNames() const {
  return trombonelexerLexerStaticData->modeNames;
}

const dfa::Vocabulary& TromboneLexer::getVocabulary() const {
  return trombonelexerLexerStaticData->vocabulary;
}

antlr4::atn::SerializedATNView TromboneLexer::getSerializedATN() const {
  return trombonelexerLexerStaticData->serializedATN;
}

const atn::ATN& TromboneLexer::getATN() const {
  return *trombonelexerLexerStaticData->atn;
}




void TromboneLexer::initialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  trombonelexerLexerInitialize();
#else
  ::antlr4::internal::call_once(trombonelexerLexerOnceFlag, trombonelexerLexerInitialize);
#endif
}
