grammar Trombone;

program     : functionDecl+ EOF;

functionDecl
    : 'fn' IDENTIFIER '(' paramList? ')' returnType? block;

paramList   : param (',' param)*;
param       : IDENTIFIER ':' type;
returnType  : '->' type;

type        : 'int' | '[' 'int' ']';

block       : '{' statement* '}';

statement
    : varDecl ';'
    | assignment ';'
    | arrayAssignment ';'
    | funcCall ';'
    | returnStmt ';'
    | whileStmt
    | ifStmt;

varDecl     : 'let' IDENTIFIER ':' type '=' expr;
assignment  : IDENTIFIER '=' expr;
arrayAssignment : IDENTIFIER '[' expr ']' '=' expr;
returnStmt  : 'return' expr;

whileStmt   : 'while' expr block;

ifStmt
    : 'if' expr block
      ('else' 'if' expr block)*
      ('else' block)?;

funcCall    : IDENTIFIER '(' argList? ')';

argList     : expr (',' expr)*;

expr
    : expr op=('*'|'/') expr       # MulDiv
    | expr op=('+'|'-') expr       # AddSub
    | expr op=('<'|'>'|'<='|'>='|'=='|'!=') expr # Compare
    | IDENTIFIER '[' expr ']'      # ArrayAccess
    | funcCall                     # FuncCallExpr
    | 'read' '(' ')'               # ReadExpr
    | 'print' '(' expr ')'         # PrintExpr
    | 'array' '(' expr (',' expr)? ')' # ArrayCreate
    | '(' expr ')'                 # Parens
    | NUMBER                       # IntLiteral
    | IDENTIFIER                   # VarReference
    ;

IDENTIFIER  : [a-zA-Z_][a-zA-Z_0-9]*;
NUMBER      : [0-9]+;

WS          : [ \t\r\n]+ -> skip;
COMMENT     : '//' ~[\r\n]* -> skip;
