
// Generated from ./antlr/Trombone.g4 by ANTLR 4.13.2

#pragma once


#include "antlr4-runtime.h"
#include "TromboneListener.h"


/**
 * This class provides an empty implementation of TromboneListener,
 * which can be extended to create a listener which only needs to handle a subset
 * of the available methods.
 */
class  TromboneBaseListener : public TromboneListener {
public:

  virtual void enterProgram(TromboneParser::ProgramContext * /*ctx*/) override { }
  virtual void exitProgram(TromboneParser::ProgramContext * /*ctx*/) override { }

  virtual void enterFunctionDecl(TromboneParser::FunctionDeclContext * /*ctx*/) override { }
  virtual void exitFunctionDecl(TromboneParser::FunctionDeclContext * /*ctx*/) override { }

  virtual void enterParamList(TromboneParser::ParamListContext * /*ctx*/) override { }
  virtual void exitParamList(TromboneParser::ParamListContext * /*ctx*/) override { }

  virtual void enterParam(TromboneParser::ParamContext * /*ctx*/) override { }
  virtual void exitParam(TromboneParser::ParamContext * /*ctx*/) override { }

  virtual void enterReturnType(TromboneParser::ReturnTypeContext * /*ctx*/) override { }
  virtual void exitReturnType(TromboneParser::ReturnTypeContext * /*ctx*/) override { }

  virtual void enterType(TromboneParser::TypeContext * /*ctx*/) override { }
  virtual void exitType(TromboneParser::TypeContext * /*ctx*/) override { }

  virtual void enterBlock(TromboneParser::BlockContext * /*ctx*/) override { }
  virtual void exitBlock(TromboneParser::BlockContext * /*ctx*/) override { }

  virtual void enterStatement(TromboneParser::StatementContext * /*ctx*/) override { }
  virtual void exitStatement(TromboneParser::StatementContext * /*ctx*/) override { }

  virtual void enterVarDecl(TromboneParser::VarDeclContext * /*ctx*/) override { }
  virtual void exitVarDecl(TromboneParser::VarDeclContext * /*ctx*/) override { }

  virtual void enterAssignment(TromboneParser::AssignmentContext * /*ctx*/) override { }
  virtual void exitAssignment(TromboneParser::AssignmentContext * /*ctx*/) override { }

  virtual void enterArrayAssignment(TromboneParser::ArrayAssignmentContext * /*ctx*/) override { }
  virtual void exitArrayAssignment(TromboneParser::ArrayAssignmentContext * /*ctx*/) override { }

  virtual void enterReturnStmt(TromboneParser::ReturnStmtContext * /*ctx*/) override { }
  virtual void exitReturnStmt(TromboneParser::ReturnStmtContext * /*ctx*/) override { }

  virtual void enterWhileStmt(TromboneParser::WhileStmtContext * /*ctx*/) override { }
  virtual void exitWhileStmt(TromboneParser::WhileStmtContext * /*ctx*/) override { }

  virtual void enterIfStmt(TromboneParser::IfStmtContext * /*ctx*/) override { }
  virtual void exitIfStmt(TromboneParser::IfStmtContext * /*ctx*/) override { }

  virtual void enterFuncCall(TromboneParser::FuncCallContext * /*ctx*/) override { }
  virtual void exitFuncCall(TromboneParser::FuncCallContext * /*ctx*/) override { }

  virtual void enterArgList(TromboneParser::ArgListContext * /*ctx*/) override { }
  virtual void exitArgList(TromboneParser::ArgListContext * /*ctx*/) override { }

  virtual void enterArrayAccess(TromboneParser::ArrayAccessContext * /*ctx*/) override { }
  virtual void exitArrayAccess(TromboneParser::ArrayAccessContext * /*ctx*/) override { }

  virtual void enterVarReference(TromboneParser::VarReferenceContext * /*ctx*/) override { }
  virtual void exitVarReference(TromboneParser::VarReferenceContext * /*ctx*/) override { }

  virtual void enterReadExpr(TromboneParser::ReadExprContext * /*ctx*/) override { }
  virtual void exitReadExpr(TromboneParser::ReadExprContext * /*ctx*/) override { }

  virtual void enterMulDiv(TromboneParser::MulDivContext * /*ctx*/) override { }
  virtual void exitMulDiv(TromboneParser::MulDivContext * /*ctx*/) override { }

  virtual void enterAddSub(TromboneParser::AddSubContext * /*ctx*/) override { }
  virtual void exitAddSub(TromboneParser::AddSubContext * /*ctx*/) override { }

  virtual void enterParens(TromboneParser::ParensContext * /*ctx*/) override { }
  virtual void exitParens(TromboneParser::ParensContext * /*ctx*/) override { }

  virtual void enterArrayCreate(TromboneParser::ArrayCreateContext * /*ctx*/) override { }
  virtual void exitArrayCreate(TromboneParser::ArrayCreateContext * /*ctx*/) override { }

  virtual void enterIntLiteral(TromboneParser::IntLiteralContext * /*ctx*/) override { }
  virtual void exitIntLiteral(TromboneParser::IntLiteralContext * /*ctx*/) override { }

  virtual void enterCompare(TromboneParser::CompareContext * /*ctx*/) override { }
  virtual void exitCompare(TromboneParser::CompareContext * /*ctx*/) override { }

  virtual void enterPrintExpr(TromboneParser::PrintExprContext * /*ctx*/) override { }
  virtual void exitPrintExpr(TromboneParser::PrintExprContext * /*ctx*/) override { }

  virtual void enterFuncCallExpr(TromboneParser::FuncCallExprContext * /*ctx*/) override { }
  virtual void exitFuncCallExpr(TromboneParser::FuncCallExprContext * /*ctx*/) override { }


  virtual void enterEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void exitEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void visitTerminal(antlr4::tree::TerminalNode * /*node*/) override { }
  virtual void visitErrorNode(antlr4::tree::ErrorNode * /*node*/) override { }

};

