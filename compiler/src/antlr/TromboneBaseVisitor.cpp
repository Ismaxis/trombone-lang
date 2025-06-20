
// Generated from ../antlr/Trombone.g4 by ANTLR 4.13.2


#include "TromboneBaseVisitor.h"
#include <any>
#include <cstdint>


std::any TromboneBaseVisitor::visitProgram(TromboneParser::ProgramContext *ctx) {
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitFunctionDecl(TromboneParser::FunctionDeclContext *ctx) {
    //TODO: implement
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitParamList(TromboneParser::ParamListContext *ctx) {
    //TODO: implement
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitParam(TromboneParser::ParamContext *ctx) {
    //TODO: implement
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitReturnType(TromboneParser::ReturnTypeContext *ctx) {
    //TODO: implement
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitType(TromboneParser::TypeContext *ctx) {
    std::string type = ctx->getText();
    if ("int" == type) {
        return std::make_any<tromb_t>(tromb_t::int_t);
    } else if (type == "[int]") {
        return std::make_any<tromb_t>(tromb_t::int_array_t);
    }
    throw std::runtime_error("Unknown type: " + type);
    return std::any();
}

std::any TromboneBaseVisitor::visitBlock(TromboneParser::BlockContext *ctx) {
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitStatement(TromboneParser::StatementContext *ctx) {
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitVarDecl(TromboneParser::VarDeclContext *ctx) {
    auto name = ctx->IDENTIFIER()->getText();
    tromb_t type = std::any_cast<tromb_t>(ctx->type()->accept(this));
    
    if (type == tromb_t::int_t) {
        ctx->expr()->accept(this);
        symbolTable[name] = varMeta(nextAddress - 1, type);
    } else if (type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented");
    }
    return std::any();
}


std::any TromboneBaseVisitor::visitAssignment(TromboneParser::AssignmentContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    if (symbolTable.find(name) == symbolTable.end()) {
        throw std::runtime_error("Unknown variable: " + name);
    }
    auto meta = symbolTable[name];
    ctx->expr()->accept(this);
    if (meta.type == tromb_t::int_t) {
        uint32_t address = popAddress() - meta.address;
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_local_store, 1);
    } else if (meta.type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented");
    }
    return std::any();
}

std::any TromboneBaseVisitor::visitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitReturnStmt(TromboneParser::ReturnStmtContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitWhileStmt(TromboneParser::WhileStmtContext *ctx) {
    std::size_t start = bytecode.tellp();
    ctx->expr()->accept(this);
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_jmp_if_not, 1);
    popAddress();

    std::size_t block_start = bytecode.tellp();
    ctx->block()->accept(this);
    std::size_t block_end = bytecode.tellp();
    int32_t address = -(block_end - start) / 8;
    bytecode.write((const char*)&address, 4);
    bytecode.write((const char*)&reserved, 3);
    bytecode.write((const char*)&op_jmp, 1);

    bytecode.seekp(block_start - 8);
    address = (block_end - block_start) / 8 + 2;
    bytecode.write((const char*)&address, 4);
    bytecode.seekp(block_end + 8);
    return std::any();
}

std::any TromboneBaseVisitor::visitIfStmt(TromboneParser::IfStmtContext *ctx) {
    auto conditions = ctx->expr();
    std::vector<std::size_t> condition_locations;
    for (auto condition : conditions) {
        condition->accept(this);
        condition_locations.push_back(bytecode.tellp());
        bytecode.write((const char*)&reserved, 7);
        bytecode.write((const char*)&op_jmp_if, 1);
        popAddress();
    }
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_jmp, 1);
    std::vector<std::size_t> block_locations;
    for (auto block : ctx->block()) {
        std::size_t block_start = bytecode.tellp();
        block->accept(this);
        std::size_t block_end = bytecode.tellp();
        block_locations.push_back(block_end);
        if (block_locations.size() > condition_locations.size()) {
            //else
            std::size_t else_location = condition_locations.back() + 8;
            bytecode.seekp(else_location);
            int32_t address = (block_start - else_location) / 8;
            bytecode.write((const char*)&address, 4);
            bytecode.seekp(block_end);
            break;
        }
        std::size_t condition_location = condition_locations[block_locations.size() - 1];
        bytecode.seekp(condition_location);
        int32_t address = (block_start - condition_location) / 8;
        bytecode.write((const char*)&address, 4);
        bytecode.seekp(block_end);
        
        bytecode.write((const char*)&reserved, 7);
        bytecode.write((const char*)&op_jmp, 1);
    }
    std::size_t current_location = bytecode.tellp();
    if (block_locations.size() == condition_locations.size()) {
        //no else
        std::size_t if_end_location = condition_locations.back() + 8;
        bytecode.seekp(if_end_location);
        int32_t address = (current_location - if_end_location) / 8;
        bytecode.write((const char*)&address, 4);
    }
    for (std::size_t i = 0; i < condition_locations.size(); ++i) {
        bytecode.seekp(block_locations[i]);
        int32_t address = (current_location - block_locations[i]) / 8;
        bytecode.write((const char*)&address, 4);
    }
    bytecode.seekp(current_location);
    return std::any();
}

std::any TromboneBaseVisitor::visitFuncCall(TromboneParser::FuncCallContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitArgList(TromboneParser::ArgListContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitArrayAccess(TromboneParser::ArrayAccessContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitVarReference(TromboneParser::VarReferenceContext *ctx) {
    auto name = ctx->IDENTIFIER()->getText();
    if (symbolTable.find(name) == symbolTable.end()) {
        throw std::runtime_error("Unknown variable: " + name);
    }
    auto meta = symbolTable[name];
    if (meta.type == tromb_t::int_t) {
        uint32_t address = pushAddress() - meta.address;
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_local_copy, 1);
    } else if (meta.type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented");
    }
    return std::any();
}

std::any TromboneBaseVisitor::visitReadExpr(TromboneParser::ReadExprContext *ctx) {
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_read, 1);
    pushAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitMulDiv(TromboneParser::MulDivContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    bytecode.write((const char*)&reserved, 7);
    std::string op = ctx->op->getText();
    if (op == "*") {
        bytecode.write((const char*)&op_mul, 1);
    } else if (op == "/") {
        bytecode.write((const char*)&op_div, 1);
    } else {
        throw std::runtime_error("Unknown operator: " + op);
    }
    popAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitAddSub(TromboneParser::AddSubContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    bytecode.write((const char*)&reserved, 7);
    std::string op = ctx->op->getText();
    if (op == "+") {
        bytecode.write((const char*)&op_add, 1);
    } else if (op == "-") {
        bytecode.write((const char*)&op_sub, 1);
    } else {
        throw std::runtime_error("Unknown operator: " + op);
    }
    popAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitParens(TromboneParser::ParensContext *ctx) {
    return visitChildren(ctx);
}

std::any TromboneBaseVisitor::visitArrayCreate(TromboneParser::ArrayCreateContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

std::any TromboneBaseVisitor::visitIntLiteral(TromboneParser::IntLiteralContext *ctx) {
    int32_t val = std::stoi(ctx->NUMBER()->getText());
    bytecode.write((const char*)&val, 4);
    bytecode.write((const char*)&reserved, 3);
    bytecode.write((const char*)&op_push, 1);
    pushAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitCompare(TromboneParser::CompareContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    bytecode.write((const char*)&reserved, 7);
    std::string op = ctx->op->getText();
    if (op == "==") {
        bytecode.write((const char*)&op_eq, 1);
    } else if (op == "!=") {
        bytecode.write((const char*)&op_ne, 1);
    } else if (op == "<") {
        bytecode.write((const char*)&op_lt, 1);
    } else if (op == ">") {
        bytecode.write((const char*)&op_gt, 1);
    } else if (op == "<=") {
        bytecode.write((const char*)&op_le, 1);
    } else if (op == ">=") {
        bytecode.write((const char*)&op_ge, 1);
    } else {
        throw std::runtime_error("Unknown operator: " + op);
    }
    popAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitPrintExpr(TromboneParser::PrintExprContext *ctx) {
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_print, 1);
    popAddress();
    return std::any();
}

std::any TromboneBaseVisitor::visitFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) {
    //TODO: implement
    throw std::runtime_error("Not implemented");
    return std::any();
}

TromboneBaseVisitor::address_t TromboneBaseVisitor::pushAddress() {
    return nextAddress++;
}

TromboneBaseVisitor::address_t TromboneBaseVisitor::popAddress() {
    return --nextAddress;
}
