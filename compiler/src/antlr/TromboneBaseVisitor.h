
// Generated from ../../antlr/Trombone.g4 by ANTLR 4.13.2

#pragma once


#include "antlr4-runtime.h"
#include "TromboneVisitor.h"


/**
 * This class provides an empty implementation of TromboneVisitor, which can be
 * extended to create a visitor which only needs to handle a subset of the available methods.
 */
class  TromboneBaseVisitor : public TromboneVisitor {
public:

  virtual std::any visitProgram(TromboneParser::ProgramContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitFunctionDecl(TromboneParser::FunctionDeclContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitParamList(TromboneParser::ParamListContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitParam(TromboneParser::ParamContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitReturnType(TromboneParser::ReturnTypeContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitType(TromboneParser::TypeContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitBlock(TromboneParser::BlockContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitStatement(TromboneParser::StatementContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitVarDecl(TromboneParser::VarDeclContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitAssignment(TromboneParser::AssignmentContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitReturnStmt(TromboneParser::ReturnStmtContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitPrintStmt(TromboneParser::PrintStmtContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitWhileStmt(TromboneParser::WhileStmtContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitIfStmt(TromboneParser::IfStmtContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitFuncCall(TromboneParser::FuncCallContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitArgList(TromboneParser::ArgListContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitArrayAccess(TromboneParser::ArrayAccessContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitVarReference(TromboneParser::VarReferenceContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitReadExpr(TromboneParser::ReadExprContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitMulDiv(TromboneParser::MulDivContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitAddSub(TromboneParser::AddSubContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitParens(TromboneParser::ParensContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitArrayCreate(TromboneParser::ArrayCreateContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitUnaryMinus(TromboneParser::UnaryMinusContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitIntLiteral(TromboneParser::IntLiteralContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitCompare(TromboneParser::CompareContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) override {
    return visitChildren(ctx);
  }


};

