#include "trombone_compiler_visitor.h"

#include <any>
#include <cstdint>
#include <filesystem>
#include <string>
#include <sys/types.h>

static constexpr std::string_view default_output_file = "./out/out.trbc";
static constexpr auto default_flags = std::ofstream::out | std::ofstream::trunc | std::ofstream::binary;

trombone_compiler_visitor::trombone_compiler_visitor()
    : trombone_compiler_visitor(std::string(default_output_file)) {}

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
    write_instruction(op_jmp);
    last_basic_block = bytecode.tellp();
    write_instruction(op_basicblock_start);
    visitChildren(ctx);
    auto main_meta = get_function("main");
    write_on_address(main_meta.address / 8, 0);
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
        write_instruction(op_local_store, address);
    } else if (meta.type == tromb_t::int_array_t) {
        //TODO: implement array
        throw std::runtime_error("Not implemented assignment to array");
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitArrayAssignment(TromboneParser::ArrayAssignmentContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);
    ctx->expr(1)->accept(this);  // value
    ctx->expr(0)->accept(this);  // offset
    if (meta.type == tromb_t::int_array_t) {
        // uint32_t address = next_address - 1 - meta.address; // different logic for instruction
        // pop_address();
        // pop_address();

        pop_address();
        pop_address();
        uint32_t address = next_address - 1 - meta.address;  // different logic for instruction
        write_instruction(op_heap_store_ptr, address);
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitReturnStmt(TromboneParser::ReturnStmtContext *ctx) {
    if (ctx->expr() != nullptr) {
        ctx->expr()->accept(this);
        // result on top of the stack
        uint32_t address = symbol_table.back().start_rsp + 1;
        write_instruction(op_local_store, address);
    }

    for (int i = symbol_table.size() - 1; i >= 0; i--) {
        clear_scope(symbol_table[i].variables);
    }

    write_nojit_instruction(op_ret);
    return std::any();
}

std::any trombone_compiler_visitor::visitPrintStmt(TromboneParser::PrintStmtContext *ctx) {
    ctx->expr()->accept(this);
    write_nojit_instruction(op_print);
    pop_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitWhileStmt(TromboneParser::WhileStmtContext *ctx) {
    std::size_t start = bytecode.tellp();
    ctx->expr()->accept(this);
    std::size_t condition_jump = bytecode.tellp();
    write_nojit_instruction(op_jmp_if_not);
    pop_address();

    ctx->block()->accept(this);
    std::size_t block_end = bytecode.tellp();
    write_nojit_instruction(op_jmp, -(block_end - start) / 8);

    write_on_address((block_end - condition_jump) / 8 + 1, condition_jump);
    return std::any();
}

std::any trombone_compiler_visitor::visitIfStmt(TromboneParser::IfStmtContext *ctx) {
    std::vector<std::size_t> condition_locations;
    for (auto condition : ctx->expr()) {
        condition->accept(this);
        condition_locations.push_back(bytecode.tellp());
        write_nojit_instruction(op_jmp_if);
        pop_address();
    }
    std::size_t if_end_location = bytecode.tellp();
    write_nojit_instruction(op_jmp);
    std::vector<std::size_t> block_locations;
    for (auto block : ctx->block()) {
        std::size_t block_start = bytecode.tellp();
        block->accept(this);
        std::size_t block_end = bytecode.tellp();
        block_locations.push_back(block_end);
        if (block_locations.size() > condition_locations.size()) {
            //else
            write_on_address((block_start - if_end_location) / 8, if_end_location);
        } else {
            std::size_t condition_location = condition_locations[block_locations.size() - 1];
            write_on_address((block_start - condition_location) / 8, condition_location);
            write_nojit_instruction(op_jmp);
        }
    }
    std::size_t current_location = bytecode.tellp();
    if (block_locations.size() == condition_locations.size()) {
        //no else
        write_on_address((current_location - if_end_location) / 8, if_end_location);
    }
    for (std::size_t i = 0; i < condition_locations.size(); ++i) {
        write_on_address((current_location - block_locations[i]) / 8, block_locations[i]);
    }
    bytecode.seekp(current_location);
    return std::any();
}

std::any trombone_compiler_visitor::visitFuncCall(TromboneParser::FuncCallContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_function(name);
    std::uint32_t argcount = ctx->argList()->expr().size();
    if (meta.return_type.has_value()) {
        write_instruction(op_push);
        push_address(); // reserve space for return value
    }
    write_instruction(op_push);
    push_address(); // reserve space for return address

    // Just evaluate arguments, do not do any heap copy for arrays
    ctx->argList()->accept(this);

    write_on_address((static_cast<uint32_t>(bytecode.tellp() - last_basic_block) / 8 - 1), last_basic_block);
    
    write_instruction(op_push_ret_address, argcount);
    write_instruction(op_jmp, (meta.address - static_cast<int>(bytecode.tellp())) / 8);
    
    last_basic_block = bytecode.tellp();
    write_instruction(op_basicblock_start);
    return std::any();
}

std::any trombone_compiler_visitor::visitFuncCallStmt(TromboneParser::FuncCallStmtContext *ctx) {
    std::string fname = ctx->funcCall()->IDENTIFIER()->getText();
    auto meta = get_function(fname);
    visitChildren(ctx);
    if (meta.return_type.has_value()) {
        write_instruction(op_pop);
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
}

std::any trombone_compiler_visitor::visitArrayAccess(TromboneParser::ArrayAccessContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);

    if (meta.type == tromb_t::int_t) {
        throw std::runtime_error("Array expected, got int");
    } else if (meta.type == tromb_t::int_array_t) {
        ctx->expr()->accept(this);
        write_instruction(op_heap_load_ptr, next_address - meta.address - 1);
    }
    return std::any();
}

std::any trombone_compiler_visitor::visitVarReference(TromboneParser::VarReferenceContext *ctx) {
    std::string name = ctx->IDENTIFIER()->getText();
    auto meta = get_variable(name);

    // Use op_local_copy for both int and array variables
    write_instruction(op_local_copy, next_address - meta.address - 1);
    push_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitReadExpr(TromboneParser::ReadExprContext *ctx) {
    write_nojit_instruction(op_read);
    push_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitMulDiv(TromboneParser::MulDivContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    std::string op = ctx->op->getText();
    if (op == "*") {
        write_instruction(op_mul);
    } else if (op == "/") {
        write_instruction(op_div);
    } else {
        throw std::runtime_error("Unknown operator: " + op);
    }
    pop_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitAddSub(TromboneParser::AddSubContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    std::string op = ctx->op->getText();
    if (op == "+") {
        write_instruction(op_add);
    } else if (op == "-") {
        write_instruction(op_sub);
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
    write_instruction(op_neg);
    return std::any();
}

std::any trombone_compiler_visitor::visitArrayCreate(TromboneParser::ArrayCreateContext *ctx) {
    auto exprs = ctx->expr();
    if (exprs.size() != 1) {
        throw std::runtime_error("Default value of array is not supported");
    }

    ctx->expr(0)->accept(this);
    write_nojit_instruction(op_heap_alloc);
    return std::any();
}

std::any trombone_compiler_visitor::visitIntLiteral(TromboneParser::IntLiteralContext *ctx) {
    uint32_t val = std::stoi(ctx->NUMBER()->getText());
    write_instruction(op_push, val);
    push_address();
    return std::any();
}

std::any trombone_compiler_visitor::visitCompare(TromboneParser::CompareContext *ctx) {
    ctx->expr(0)->accept(this);
    ctx->expr(1)->accept(this);
    std::string op = ctx->op->getText();
    if (op == "==") {
        write_instruction(op_eq);
    } else if (op == "!=") {
        write_instruction(op_ne);
    } else if (op == "<") {
        write_instruction(op_lt);
    } else if (op == ">") {
        write_instruction(op_gt);
    } else if (op == "<=") {
        write_instruction(op_le);
    } else if (op == ">=") {
        write_instruction(op_ge);
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
