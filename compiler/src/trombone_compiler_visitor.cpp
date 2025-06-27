#include "trombone_compiler_visitor.h"

#include <any>
#include <cstdint>
#include <filesystem>
#include <string>
#include <sys/types.h>

static constexpr std::string_view default_output_file = "./out/out.trbc";
static constexpr auto default_flags = std::ofstream::out | std::ofstream::trunc | std::ofstream::binary;

trombone_compiler_visitor::trombone_compiler_visitor()
    : bytecode(std::string(default_output_file), default_flags) {}

trombone_compiler_visitor::trombone_compiler_visitor(std::string output_file) {
    std::filesystem::path output_path(output_file);

    try {
        if (!std::filesystem::exists(output_path.parent_path())) {
            std::filesystem::create_directories(output_path.parent_path());
        }
        bytecode = std::ofstream(output_file, default_flags);
    } catch (const std::filesystem::filesystem_error &e) {
        std::cerr << "Error creating output directory: " << e.what() << std::endl;
        bytecode = std::ofstream(std::string(default_output_file), default_flags);
    }
}
std::any trombone_compiler_visitor::visitProgram(TromboneParser::ProgramContext *ctx) {
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_jmp, 1);
    visitChildren(ctx);
    auto main_meta = get_function("main");
    uint32_t address = main_meta.address / 8;
    bytecode.seekp(0);
    bytecode.write((const char*)&address, 4);
    return std::any();
}

std::any trombone_compiler_visitor::visitFunctionDecl(TromboneParser::FunctionDeclContext *ctx) {
    auto return_ctx = ctx->returnType();
    std::optional<tromb_t> return_type;
    if (return_ctx) {
        return_type = std::any_cast<tromb_t>(ctx->returnType()->accept(this));
    }

    func_meta meta(bytecode.tellp(), return_type);
    std::string name = ctx->IDENTIFIER()->getText();
    add_function(name, meta);

    enter_scope();
    auto param_ctx = ctx->paramList();
    if (param_ctx) {
        param_ctx->accept(this);
    }
    std::cout << __LINE__ << ": " << next_address << std::endl;
    // symbol_table.back().start_rsp = next_address;

    ctx->block()->accept(this);
    exit_scope();

    return std::any();
}

std::any trombone_compiler_visitor::visitParamList(TromboneParser::ParamListContext *ctx) {
    return visitChildren(ctx);
}

std::any trombone_compiler_visitor::visitParam(TromboneParser::ParamContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    tromb_t type = std::any_cast<tromb_t>(ctx->type()->accept(this));
    add_variable(name, var_meta(push_address(), type));
    return std::any();
}

std::any trombone_compiler_visitor::visitReturnType(TromboneParser::ReturnTypeContext *ctx) {
    return visitChildren(ctx);
}

std::any trombone_compiler_visitor::visitType(TromboneParser::TypeContext *ctx) {
    std::string type = ctx->getText();
    if ("int" == type) {
        return std::make_any<tromb_t>(tromb_t::int_t);
    } else if (type == "[int]") {
        return std::make_any<tromb_t>(tromb_t::int_array_t);
    }
    throw std::runtime_error("Unknown type: " + type);
    return std::any();
}

std::any trombone_compiler_visitor::visitBlock(TromboneParser::BlockContext *ctx) {
    enter_scope();
    visitChildren(ctx);
    clear_scope(); // useless when block is the main block of the func, because return will clear the scopes
    exit_scope();
    return std::any();
}

std::any trombone_compiler_visitor::visitStatement(TromboneParser::StatementContext *ctx) {
    return visitChildren(ctx);
}

std::any trombone_compiler_visitor::visitVarDecl(TromboneParser::VarDeclContext *ctx) {
    auto name = ctx->IDENTIFIER()->getText();
    tromb_t type = std::any_cast<tromb_t>(ctx->type()->accept(this));
    ctx->expr()->accept(this);
    add_variable(name, var_meta(next_address - 1, type));
    return std::any();
}


std::any trombone_compiler_visitor::visitAssignment(TromboneParser::AssignmentContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);
    ctx->expr()->accept(this);
    if (meta.type == tromb_t::int_t) {
        std::cout << __LINE__ << "assign: " << name << ": " << meta.address << std::endl;
        uint32_t address = pop_address() - meta.address - 1;
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_local_store, 1);
    } else if (meta.type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented assignment to array");
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);
    ctx->expr(1)->accept(this);
    ctx->expr(0)->accept(this);
    if (meta.type == tromb_t::int_array_t) {
        uint32_t address = next_address - 1 - meta.address; // different logic for instruction
        pop_address();
        pop_address();
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_heap_store_ptr, 1);
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitReturnStmt(TromboneParser::ReturnStmtContext *ctx) {
    if (ctx->expr() != nullptr) {
        ctx->expr()->accept(this);
        // result on top of the stack
        uint32_t address = symbol_table.back().start_rsp + 1;
        // uint32_t address = next_address; // TODO: check +- 1
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_local_store, 1);
        // pop_address();
    }

    for (int i = symbol_table.size() - 1; i >= 0; i--) {
        clear_scope(symbol_table[i].variables);
    }

    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_ret, 1);
    return std::any();
}

std::any trombone_compiler_visitor::visitPrintStmt(TromboneParser::PrintStmtContext *ctx) {
    ctx->expr()->accept(this);
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_print, 1);
    pop_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitWhileStmt(TromboneParser::WhileStmtContext *ctx) {
    std::size_t start = bytecode.tellp();
    ctx->expr()->accept(this);
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_jmp_if_not, 1);
    pop_address();

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

std::any trombone_compiler_visitor::visitIfStmt(TromboneParser::IfStmtContext *ctx) {
    auto conditions = ctx->expr();
    std::vector<std::size_t> condition_locations;
    for (auto condition : conditions) {
        condition->accept(this);
        condition_locations.push_back(bytecode.tellp());
        bytecode.write((const char*)&reserved, 7);
        bytecode.write((const char*)&op_jmp_if, 1);
        pop_address();
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

std::any trombone_compiler_visitor::visitFuncCall(TromboneParser::FuncCallContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_function(name);
    std::uint32_t argcount = ctx->argList()->expr().size();
    if (meta.return_type.has_value()) {
        bytecode.write((const char *)&reserved, 7);
        bytecode.write((const char *)&op_push, 1);
        push_address(); // reserve space for return value
    }
    bytecode.write((const char *)&reserved, 7);
    bytecode.write((const char *)&op_push, 1);
    push_address(); // reserve space for return address

    ctx->argList()->accept(this);
    bytecode.write((const char *)&argcount, 4);
    bytecode.write((const char *)&reserved, 3);
    bytecode.write((const char *)&op_push_ret_address, 1);
    
    std::uint32_t address =
        (meta.address - static_cast<int>(bytecode.tellp())) / 8;
    bytecode.write((const char*)&address, 4);
    bytecode.write((const char*)&reserved, 3);
    bytecode.write((const char *)&op_jmp, 1);
    return std::any();
}

std::any trombone_compiler_visitor::visitFuncCallStmt(TromboneParser::FuncCallStmtContext *ctx) {
    std::string fname = ctx->funcCall()->IDENTIFIER()->getText();
    auto meta = get_function(fname);
    visitChildren(ctx);
    if (meta.return_type.has_value()) {
        bytecode.write((const char *)&reserved, 7);
        bytecode.write((const char *)&op_pop, 1);
        pop_address();
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitArgList(TromboneParser::ArgListContext *ctx) {
    for (auto arg : ctx->expr()) {
        arg->accept(this);
        pop_address();
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitFuncCallExpr(TromboneParser::FuncCallExprContext *ctx) {
    return visitChildren(ctx);
    //TODO: implement
    throw std::runtime_error("Not implemented visitFuncCallExpr");
    return std::any();
}

std::any trombone_compiler_visitor::visitArrayAccess(TromboneParser::ArrayAccessContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);

    if (meta.type == tromb_t::int_t) {
        throw std::runtime_error("Array expected, got int");
    } else if (meta.type == tromb_t::int_array_t) {
        ctx->expr()->accept(this);
        uint32_t address = next_address - 1 - meta.address; // different logic for instruction
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_heap_load_ptr, 1);
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitVarReference(TromboneParser::VarReferenceContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);

    if (meta.type == tromb_t::int_t) {
        uint32_t address = next_address - meta.address - 1;
        std::cout << __LINE__ << "var ref: " << name << ": " << meta.address << " " << address << std::endl;
        bytecode.write((const char*)&address, 4);
        bytecode.write((const char*)&reserved, 3);
        bytecode.write((const char*)&op_local_copy, 1);
        push_address();
    } else if (meta.type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented visitVarReference for array"); 
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitReadExpr(TromboneParser::ReadExprContext *ctx) {
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_read, 1);
    push_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitMulDiv(TromboneParser::MulDivContext *ctx) {
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
    pop_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitAddSub(TromboneParser::AddSubContext *ctx) {
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
    pop_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitParens(TromboneParser::ParensContext *ctx) {
    return visitChildren(ctx);
}

std::any trombone_compiler_visitor::visitUnaryMinus(TromboneParser::UnaryMinusContext *ctx) {
    ctx->expr()->accept(this);
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_neg, 1);
    return std::any();
}

std::any trombone_compiler_visitor::visitArrayCreate(TromboneParser::ArrayCreateContext *ctx) {
    auto exprs = ctx->expr();
    size_t size = exprs.size();
    if (size != 1) {
        throw std::runtime_error("Default value of array is not supported");
    }

    ctx->expr(0)->accept(this);
    bytecode.write((const char*)&reserved, 7);
    bytecode.write((const char*)&op_heap_alloc, 1);
    return std::any();
}

std::any trombone_compiler_visitor::visitIntLiteral(TromboneParser::IntLiteralContext *ctx) {
    int32_t val = std::stoi(ctx->NUMBER()->getText());
    bytecode.write((const char*)&val, 4);
    bytecode.write((const char*)&reserved, 3);
    bytecode.write((const char*)&op_push, 1);
    push_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitCompare(TromboneParser::CompareContext *ctx) {
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
    pop_address();
    return std::any();
}

trombone_compiler_visitor::address_t trombone_compiler_visitor::push_address() {
    return next_address++;
}

trombone_compiler_visitor::address_t trombone_compiler_visitor::pop_address() {
    return --next_address;
}
