
// Generated from ./antlr/Trombone.g4 by ANTLR 4.13.2

#pragma once


#include "antlr4-runtime.h"
#include "TromboneParser.h"


/**
 * This interface defines an abstract listener for a parse tree produced by TromboneParser.
 */
class  TromboneListener : public antlr4::tree::ParseTreeListener {
public:

  virtual void enterProgram(TromboneParser::ProgramContext *ctx) = 0;
  virtual void exitProgram(TromboneParser::ProgramContext *ctx) = 0;

  virtual void enterFunctionDecl(TromboneParser::FunctionDeclContext *ctx) = 0;
  virtual void exitFunctionDecl(TromboneParser::FunctionDeclContext *ctx) = 0;

  virtual void enterParamList(TromboneParser::ParamListContext *ctx) = 0;
  virtual void exitParamList(TromboneParser::ParamListContext *ctx) = 0;

  virtual void enterParam(TromboneParser::ParamContext *ctx) = 0;
  virtual void exitParam(TromboneParser::ParamContext *ctx) = 0;

  virtual void enterReturnType(TromboneParser::ReturnTypeContext *ctx) = 0;
  virtual void exitReturnType(TromboneParser::ReturnTypeContext *ctx) = 0;

  virtual void enterType(TromboneParser::TypeContext *ctx) = 0;
  virtual void exitType(TromboneParser::TypeContext *ctx) = 0;

  virtual void enterBlock(TromboneParser::BlockContext *ctx) = 0;
  virtual void exitBlock(TromboneParser::BlockContext *ctx) = 0;

  virtual void enterStatement(TromboneParser::StatementContext *ctx) = 0;
  virtual void exitStatement(TromboneParser::StatementContext *ctx) = 0;

  virtual void enterVarDecl(TromboneParser::VarDeclContext *ctx) = 0;
  virtual void exitVarDecl(TromboneParser::VarDeclContext *ctx) = 0;

  virtual void enterAssignment(TromboneParser::AssignmentContext *ctx) = 0;
  virtual void exitAssignment(TromboneParser::AssignmentContext *ctx) = 0;

  virtual void enterArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) = 0;
  virtual void exitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) = 0;

  virtual void enterReturnStmt(TromboneParser::ReturnStmtContext *ctx) = 0;
  virtual void exitReturnStmt(TromboneParser::ReturnStmtContext *ctx) = 0;

  virtual void enterWhileStmt(TromboneParser::WhileStmtContext *ctx) = 0;
  virtual void exitWhileStmt(TromboneParser::WhileStmtContext *ctx) = 0;

  virtual void enterIfStmt(TromboneParser::IfStmtContext *ctx) = 0;
  virtual void exitIfStmt(TromboneParser::IfStmtContext *ctx) = 0;

  virtual void enterFuncCall(TromboneParser::FuncCallContext *ctx) = 0;
  virtual void exitFuncCall(TromboneParser::FuncCallContext *ctx) = 0;

  virtual void enterArgList(TromboneParser::ArgListContext *ctx) = 0;
  virtual void exitArgList(TromboneParser::ArgListContext *ctx) = 0;

  virtual void enterArrayAccess(TromboneParser::ArrayAccessContext *ctx) = 0;
  virtual void exitArrayAccess(TromboneParser::ArrayAccessContext *ctx) = 0;

  virtual void enterVarReference(TromboneParser::VarReferenceContext *ctx) = 0;
  virtual void exitVarReference(TromboneParser::VarReferenceContext *ctx) = 0;

  virtual void enterReadExpr(TromboneParser::ReadExprContext *ctx) = 0;
  virtual void exitReadExpr(TromboneParser::ReadExprContext *ctx) = 0;

  virtual void enterMulDiv(TromboneParser::MulDivContext *ctx) = 0;
  virtual void exitMulDiv(TromboneParser::MulDivContext *ctx) = 0;

  virtual void enterAddSub(TromboneParser::AddSubContext *ctx) = 0;
  virtual void exitAddSub(TromboneParser::AddSubContext *ctx) = 0;

  virtual void enterParens(TromboneParser::ParensContext *ctx) = 0;
  virtual void exitParens(TromboneParser::ParensContext *ctx) = 0;

  virtual void enterArrayCreate(TromboneParser::ArrayCreateContext *ctx) = 0;
  virtual void exitArrayCreate(TromboneParser::ArrayCreateContext *ctx) = 0;

  virtual void enterIntLiteral(TromboneParser::IntLiteralContext *ctx) = 0;
  virtual void exitIntLiteral(TromboneParser::IntLiteralContext *ctx) = 0;

  virtual void enterCompare(TromboneParser::CompareContext *ctx) = 0;
  virtual void exitCompare(TromboneParser::CompareContext *ctx) = 0;

  virtual void enterPrintExpr(TromboneParser::PrintExprContext *ctx) = 0;
  virtual void exitPrintExpr(TromboneParser::PrintExprContext *ctx) = 0;

  virtual void enterFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) = 0;
  virtual void exitFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) = 0;


};

