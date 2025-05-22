
// Generated from ../antlr/Trombone.g4 by ANTLR 4.13.2

#pragma once


#include "antlr4-runtime.h"
#include "TromboneParser.h"



/**
 * This class defines an abstract visitor for a parse tree
 * produced by TromboneParser.
 */
class  TromboneVisitor : public antlr4::tree::AbstractParseTreeVisitor {
public:

  /**
   * Visit parse trees produced by TromboneParser.
   */
    virtual std::any visitProgram(TromboneParser::ProgramContext *context) = 0;

    virtual std::any visitFunctionDecl(TromboneParser::FunctionDeclContext *context) = 0;

    virtual std::any visitParamList(TromboneParser::ParamListContext *context) = 0;

    virtual std::any visitParam(TromboneParser::ParamContext *context) = 0;

    virtual std::any visitReturnType(TromboneParser::ReturnTypeContext *context) = 0;

    virtual std::any visitType(TromboneParser::TypeContext *context) = 0;

    virtual std::any visitBlock(TromboneParser::BlockContext *context) = 0;

    virtual std::any visitStatement(TromboneParser::StatementContext *context) = 0;

    virtual std::any visitVarDecl(TromboneParser::VarDeclContext *context) = 0;

    virtual std::any visitAssignment(TromboneParser::AssignmentContext *context) = 0;

    virtual std::any visitArrayAssignment(TromboneParser::ArrayAssignmentContext *context) = 0;

    virtual std::any visitReturnStmt(TromboneParser::ReturnStmtContext *context) = 0;

    virtual std::any visitWhileStmt(TromboneParser::WhileStmtContext *context) = 0;

    virtual std::any visitIfStmt(TromboneParser::IfStmtContext *context) = 0;

    virtual std::any visitFuncCall(TromboneParser::FuncCallContext *context) = 0;

    virtual std::any visitArgList(TromboneParser::ArgListContext *context) = 0;

    virtual std::any visitArrayAccess(TromboneParser::ArrayAccessContext *context) = 0;

    virtual std::any visitVarReference(TromboneParser::VarReferenceContext *context) = 0;

    virtual std::any visitReadExpr(TromboneParser::ReadExprContext *context) = 0;

    virtual std::any visitMulDiv(TromboneParser::MulDivContext *context) = 0;

    virtual std::any visitAddSub(TromboneParser::AddSubContext *context) = 0;

    virtual std::any visitParens(TromboneParser::ParensContext *context) = 0;

    virtual std::any visitArrayCreate(TromboneParser::ArrayCreateContext *context) = 0;

    virtual std::any visitIntLiteral(TromboneParser::IntLiteralContext *context) = 0;

    virtual std::any visitCompare(TromboneParser::CompareContext *context) = 0;

    virtual std::any visitPrintExpr(TromboneParser::PrintExprContext *context) = 0;

    virtual std::any visitFuncCallExpr(TromboneParser::FuncCallExprContext *context) = 0;


};

