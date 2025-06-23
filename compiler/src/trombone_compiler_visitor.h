
// Generated from ../antlr/Trombone.g4 by ANTLR 4.13.2

#pragma once

#include "antlr/TromboneVisitor.h"
#include "antlr4-runtime.h"
#include <any>
#include <cstdint>
#include <fstream>
#include <unordered_map>

/**
 * This class provides an empty implementation of TromboneVisitor, which can be
 * extended to create a visitor which only needs to handle a subset of the
 * available methods.
 */
class trombone_compiler_visitor : public TromboneVisitor {
public:
  trombone_compiler_visitor();
  trombone_compiler_visitor(std::string output_file);
  using address_t = uint64_t;
  enum class tromb_t { int_t, int_array_t };

  using op_code_t = uint8_t;

  virtual std::any visitProgram(TromboneParser::ProgramContext *ctx) override;
  virtual std::any
  visitFunctionDecl(TromboneParser::FunctionDeclContext *ctx) override;
  virtual std::any
  visitParamList(TromboneParser::ParamListContext *ctx) override;
  virtual std::any visitParam(TromboneParser::ParamContext *ctx) override;
  virtual std::any
  visitReturnType(TromboneParser::ReturnTypeContext *ctx) override;
  virtual std::any visitType(TromboneParser::TypeContext *ctx) override;
  virtual std::any visitBlock(TromboneParser::BlockContext *ctx) override;
  virtual std::any
  visitStatement(TromboneParser::StatementContext *ctx) override;
  virtual std::any visitVarDecl(TromboneParser::VarDeclContext *ctx) override;
  virtual std::any
  visitAssignment(TromboneParser::AssignmentContext *ctx) override;
  virtual std::any
  visitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) override;
  virtual std::any
  visitReturnStmt(TromboneParser::ReturnStmtContext *ctx) override;
  virtual std::any visitPrintStmt(TromboneParser::PrintStmtContext *ctx) override;
  virtual std::any
  visitWhileStmt(TromboneParser::WhileStmtContext *ctx) override;
  virtual std::any visitIfStmt(TromboneParser::IfStmtContext *ctx) override;
  virtual std::any visitFuncCall(TromboneParser::FuncCallContext *ctx) override;
  virtual std::any visitArgList(TromboneParser::ArgListContext *ctx) override;
  virtual std::any
  visitArrayAccess(TromboneParser::ArrayAccessContext *ctx) override;
  virtual std::any
  visitVarReference(TromboneParser::VarReferenceContext *ctx) override;
  virtual std::any visitReadExpr(TromboneParser::ReadExprContext *ctx) override;
  virtual std::any visitMulDiv(TromboneParser::MulDivContext *ctx) override;
  virtual std::any visitAddSub(TromboneParser::AddSubContext *ctx) override;
  virtual std::any visitParens(TromboneParser::ParensContext *ctx) override;
  virtual std::any visitUnaryMinus(TromboneParser::UnaryMinusContext *ctx) override;
  virtual std::any
  visitArrayCreate(TromboneParser::ArrayCreateContext *ctx) override;
  virtual std::any
  visitIntLiteral(TromboneParser::IntLiteralContext *ctx) override;
  virtual std::any visitCompare(TromboneParser::CompareContext *ctx) override;
  virtual std::any
  visitFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) override;

private:
#pragma pack(push, 1)
  static inline const op_code_t op_push = 0x01;
  static inline const op_code_t op_pop = 0x02;
  static inline const op_code_t op_local_copy = 0x03;
  static inline const op_code_t op_local_store = 0x04;
  static inline const op_code_t op_neg = 0xa0;
  static inline const op_code_t op_not = 0xa1;
  static inline const op_code_t op_add = 0xa2;
  static inline const op_code_t op_sub = 0xa3;
  static inline const op_code_t op_mul = 0xa4;
  static inline const op_code_t op_div = 0xa5;
  static inline const op_code_t op_mod = 0xa6;
  static inline const op_code_t op_and = 0xa7;
  static inline const op_code_t op_or = 0xa8;
  static inline const op_code_t op_xor = 0xa9;
  static inline const op_code_t op_lsh = 0xaa;
  static inline const op_code_t op_rsh = 0xab;
  static inline const op_code_t op_eq = 0xc0;
  static inline const op_code_t op_ne = 0xc1;
  static inline const op_code_t op_lt = 0xc2;
  static inline const op_code_t op_gt = 0xc3;
  static inline const op_code_t op_le = 0xc4;
  static inline const op_code_t op_ge = 0xc5;
  static inline const op_code_t op_jmp = 0xd0;
  static inline const op_code_t op_jmp_if = 0xd1;
  static inline const op_code_t op_jmp_if_not = 0xd2;
  static inline const op_code_t op_heap_alloc = 0xe0;
  static inline const op_code_t op_heap_pop_ptr = 0xe1;
  static inline const op_code_t op_heap_copy_ptr = 0xe2;
  static inline const op_code_t op_heap_load_ptr = 0xe3;
  static inline const op_code_t op_heap_store_ptr = 0xe4;
  static inline const op_code_t op_read = 0xf0;
  static inline const op_code_t op_print = 0xf1;

#pragma pack(pop)
  struct var_meta {
    address_t address;
    tromb_t type;
    var_meta() {}
    var_meta(address_t address, tromb_t type) : address(address), type(type) {}
    var_meta(const var_meta &other)
        : address(other.address), type(other.type) {}
    var_meta &operator=(const var_meta &other) {
      address = other.address;
      type = other.type;
      return *this;
    }
  };

  static inline const uint64_t reserved = 0;
  std::ofstream bytecode;

  std::unordered_map<std::string, var_meta> symbol_table;
  address_t next_address = 0;

  address_t push_address();
  address_t pop_address();
};
