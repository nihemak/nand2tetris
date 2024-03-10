#include "CompilationEngine.h"
#include "JackTokenizer.h"
#include "SymbolTable.h"
#include "VMWriter.h"
#include "string.h"
#include <stdarg.h>

void getIdentifierKindString(SymbolTable symbolTable, char *token, char *category);
void getKeywordString(JackTokenizer tokenizer, char *keyword);
bool isKeywordToken(CompilationEngine thisObject, JackTokenizer_Keyword keyword);
bool isSymbolToken(CompilationEngine thisObject, char *symbol);
bool inSymbolListToken(CompilationEngine thisObject, ...);

typedef struct compilation_engine * CompilationEngine;
struct compilation_engine
{
    JackTokenizer tokenizer;
    SymbolTable symbolTable;
    WMWriter vmWriter;
    char className[JACK_TOKEN_SIZE];
};

CompilationEngine CompilationEngine_init(FILE *fpJack, FILE *fpVm)
{
    static struct compilation_engine thisObject;

    thisObject.tokenizer = JackTokenizer_init(fpJack);
    thisObject.symbolTable = SymbolTable_init();
    thisObject.vmWriter = WMWriter_init(fpVm);
    strcpy(thisObject.className, "");

    return &thisObject;
}

// 'class' className '{' classVarDec* subroutineDec* '}'
void CompilationEngine_compileClass(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    JackTokenizer_advance(thisObject->tokenizer);

    // 'class'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // className (class defined)
    JackTokenizer_identifier(thisObject->tokenizer, thisObject->className);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // '}' or not
    while (! isSymbolToken(thisObject, "}")) {
        if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_STATIC) ||
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_FIELD)) {
            // classVarDec
            CompilationEngine_compileClassVarDec(thisObject);
        } else if ( isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_CONSTRUCTION) ||
                    isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_FUNCTION) ||
                    isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_METHOD)) {
            // subroutineDec
            CompilationEngine_compileSubroutine(thisObject);
        } else {
            break;
        }
    }

    // '}'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);

    VMWriter_close(thisObject->vmWriter);

    SymbolTable_delete(thisObject->symbolTable);
    thisObject->symbolTable = SymbolTable_init();
}

// ('static' | 'field') type varName (',' varName)* ';'
void CompilationEngine_compileClassVarDec(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // ('static' | 'field')
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    SymbolTable_Kind kind = keyword == JACK_TOKENIZER_KEYWORD_STATIC ? SYMBOL_TABLE_KIND_STATIC : SYMBOL_TABLE_KIND_FIELD;
    JackTokenizer_advance(thisObject->tokenizer); 

    // type
    char type[JACK_TOKEN_SIZE];
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        // class used
        JackTokenizer_identifier(thisObject->tokenizer, type);
    } else {
        keyword = JackTokenizer_keyword(thisObject->tokenizer);
        getKeywordString(thisObject->tokenizer, type);
    }

    do {
        JackTokenizer_advance(thisObject->tokenizer);

        // varName
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(thisObject->symbolTable, varName, type, kind);
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or ';'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    } while (! isSymbolToken(thisObject, ";"));
    JackTokenizer_advance(thisObject->tokenizer);
}

// ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
// subroutineBody: '{' varDec* statements '}'
void CompilationEngine_compileSubroutine(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char identifier[JACK_TOKEN_SIZE];
    char symbol[JACK_TOKEN_SIZE];

    SymbolTable_startSubroutine(thisObject->symbolTable);

    // subroutineDec

    // ('constructor' | 'function' | 'method')
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // ('void' | type)
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        // class used
        JackTokenizer_identifier(thisObject->tokenizer, identifier);
    } else {
        keyword = JackTokenizer_keyword(thisObject->tokenizer);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineName (subroutine defined)
    char subroutineName[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, subroutineName);
    JackTokenizer_advance(thisObject->tokenizer);

    char functionName[JACK_TOKEN_SIZE];
    sprintf(functionName, "%s.%s", thisObject->className, subroutineName);
    VMWriter_writeFunction(thisObject->vmWriter, functionName, 0 /* FIXME */);

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileParameterList(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineBody
    {
        // '{'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        // 'var' or statements or '}'
        while (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_VAR)) {
            CompilationEngine_compileVarDec(thisObject);
        }
        // statements or '}'
        if (! isSymbolToken(thisObject, "}")) {
            CompilationEngine_compileStatements(thisObject);
        }

        // '}'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);
    }
}

// ((type varName) (',' type varName)*)?
void CompilationEngine_compileParameterList(CompilationEngine thisObject)
{
    char symbol[JACK_TOKEN_SIZE];

    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD || 
        JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        // type
        char type[JACK_TOKEN_SIZE];
        if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
            //　class used
            JackTokenizer_identifier(thisObject->tokenizer, type);
        } else {
            getKeywordString(thisObject->tokenizer, type);
        }
        JackTokenizer_advance(thisObject->tokenizer);

        // varName (argument defined)
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_ARG);
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or not
        while (isSymbolToken(thisObject, ",")) {
            // ','
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            // type
            if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
                // class used
                JackTokenizer_identifier(thisObject->tokenizer, type);
            } else {
                getKeywordString(thisObject->tokenizer, type);
            }
            JackTokenizer_advance(thisObject->tokenizer);

            // varName (argument defined)
            JackTokenizer_identifier(thisObject->tokenizer, varName);
            SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_ARG);
            JackTokenizer_advance(thisObject->tokenizer);
        }
    }
}

// 'var' type varName (',' varName)* ';'
void CompilationEngine_compileVarDec(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // 'var'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // type
    char type[JACK_TOKEN_SIZE];
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        // class used
        JackTokenizer_identifier(thisObject->tokenizer, type);
    } else {
        getKeywordString(thisObject->tokenizer, type);
    }

    do {
        JackTokenizer_advance(thisObject->tokenizer);

        // varName (var defined)
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_VAR);
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or ';'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    } while (! isSymbolToken(thisObject, ";"));
    JackTokenizer_advance(thisObject->tokenizer);
}

// statement*
// statement: letStatement | ifStatement | whileStatement | doStatement | returnStatement
void CompilationEngine_compileStatements(CompilationEngine thisObject)
{
    do {
        if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_LET)) {
            CompilationEngine_compileLet(thisObject);
        } else if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_IF)) {
            CompilationEngine_compileIf(thisObject);
        } else if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_WHILE)) {
            CompilationEngine_compileWhile(thisObject);
        } else if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_DO)) {
            CompilationEngine_compileDo(thisObject);
        } else if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_RETURN)) {
            CompilationEngine_compileReturn(thisObject);
        } else {
            break;
        }
    } while (true);
}

// 'do' subroutineCall ';'
// subroutineCall: subroutineName '(' expressionList ')' | (className | varName) '.' subroutineName '(' expressionList ')'
void CompilationEngine_compileDo(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char identifier[JACK_TOKEN_SIZE];
    char symbol[JACK_TOKEN_SIZE];

    char functionName[JACK_TOKEN_SIZE];

    // 'do'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineName | (className | varName)
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, token);
    JackTokenizer_advance(thisObject->tokenizer);

    sprintf(functionName, "%s", token);

    // '(' or '.'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    if (isSymbolToken(thisObject, ".")) {
        // (className | varName) used
        char category[JACK_TOKEN_SIZE];
        getIdentifierKindString(thisObject->symbolTable, token, category);
        if (strcmp(category, "none") == 0) {
            // token is className
            strcpy(category, "class");
        } else {
            // token is varName
        }
        JackTokenizer_advance(thisObject->tokenizer);

        // subroutineName (subroutine used)
        JackTokenizer_identifier(thisObject->tokenizer, identifier);
        JackTokenizer_advance(thisObject->tokenizer);

        sprintf(functionName, "%s%s%s", functionName, symbol, identifier);

        // '('
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    } else {
        // token is subroutineName (subroutine used)
    }
    JackTokenizer_advance(thisObject->tokenizer);

    int nArgs = CompilationEngine_compileExpressionList(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // ';'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writeCall(thisObject->vmWriter, functionName, nArgs);
    VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_TEMP, 0);
}

// 'let' varName ('[' expression ']')? '=' expression ';'
void CompilationEngine_compileLet(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // 'let'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // varName (used)
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, token);
    char category[JACK_TOKEN_SIZE];
    getIdentifierKindString(thisObject->symbolTable, token, category);
    JackTokenizer_advance(thisObject->tokenizer);

    // '[' or '='
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    if (isSymbolToken(thisObject, "[")) {
        JackTokenizer_advance(thisObject->tokenizer);
    
        CompilationEngine_compileExpression(thisObject);

        // ']'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        // '='
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ';'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);
}

// 'while' '(' expression ')' '{' statements '}'
void CompilationEngine_compileWhile(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // 'while'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    // '}'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);
}

// 'return' expression? ';'
void CompilationEngine_compileReturn(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // 'return'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // expression or ';'
    if (! isSymbolToken(thisObject, ";")) {
        CompilationEngine_compileExpression(thisObject);
    }

    // ';'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, 0);  /* FIXME */
    VMWriter_writeReturn(thisObject->vmWriter);
}

// 'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?
void CompilationEngine_compileIf(CompilationEngine thisObject)
{
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];

    // 'if'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    // '}'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    // 'else' or not
    if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_ELSE)) {
        // 'else'
        keyword = JackTokenizer_keyword(thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        // '{'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileStatements(thisObject);

        // '}'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);
    }
}

// term (op term)*
// op: '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='
void CompilationEngine_compileExpression(CompilationEngine thisObject)
{
    char symbol[JACK_TOKEN_SIZE];

    CompilationEngine_compileTerm(thisObject);

    while (inSymbolListToken(thisObject, "+", "-", "*",  "/", "&", "|", "<", ">", "=", NULL)) {
        // op
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileTerm(thisObject);

        // op is after terms because it is Reverse Polish Notation (RPN)
        // term1 term2 op (ex. 1+(2*3) => 1(2*3)+ => 1(23*)+)
        if (strcmp(symbol, "+") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_ADD);
        }
        if (strcmp(symbol, "*") == 0) {
            VMWriter_writeCall(thisObject->vmWriter, "Math.multiply", 2);
        }
    }
}

// integerConstant | stringConstant | keywordConstant | varName | varName '[' expression ']' | subroutineCall | '(' expression ')' | unaryOp term
// subroutineCall: subroutineName '(' expressionList ')' | (className | varName) '.' subroutineName '(' expressionList ')'
// unaryOp: '-' | '~'
void CompilationEngine_compileTerm(CompilationEngine thisObject)
{
    int intVal;
    char stringVal[JACK_TOKEN_SIZE];
    JackTokenizer_Keyword keyword;
    char symbol[JACK_TOKEN_SIZE];
    char identifier[JACK_TOKEN_SIZE];

    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_INT_CONST) {
        // integerConstant
        JackTokenizer_intVal(thisObject->tokenizer, &intVal);
        JackTokenizer_advance(thisObject->tokenizer);

        VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, intVal);
    } else if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST) {
        // stringConstant
        JackTokenizer_stringVal(thisObject->tokenizer, stringVal);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD) {
        // keywordConstant
        keyword = JackTokenizer_keyword(thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (isSymbolToken(thisObject, "(")) {    // '(' expression ')'
        // '('
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileExpression(thisObject);

        // ')'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (inSymbolListToken(thisObject, "-", "~", NULL)) { // unaryOp term
        // unaryOp
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileTerm(thisObject);
    } else {    // varName | varName '[' expression ']' | subroutineCall
        // varName | subroutineName | className (used)
        char token[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, token);
        char category[JACK_TOKEN_SIZE];
        getIdentifierKindString(thisObject->symbolTable, token, category);
        if (strcmp(category, "none") == 0) {
            // token is className
            strcpy(category, "class");
        }
        JackTokenizer_advance(thisObject->tokenizer);

        // '[' or '(' or '.' or not
        if (isSymbolToken(thisObject, "[")) {
            // token is Array of varName (varName[])

            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            CompilationEngine_compileExpression(thisObject);

            // ']'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);
        } else if (inSymbolListToken(thisObject, "(", ".", NULL)) {
            if (isSymbolToken(thisObject, "(")) {
                // token is subroutineName (subroutine used)
            } else {    // "."
                // token is (className | varName)
            }

            // '(' or '.'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            if (isSymbolToken(thisObject, ".")) {
                JackTokenizer_advance(thisObject->tokenizer);

                // subroutineName (subroutine used)
                JackTokenizer_identifier(thisObject->tokenizer, identifier);
                JackTokenizer_advance(thisObject->tokenizer);

                // '('
                JackTokenizer_symbol(thisObject->tokenizer, symbol);
            }
            JackTokenizer_advance(thisObject->tokenizer);

            CompilationEngine_compileExpressionList(thisObject);

            // ')'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);
        } else {
            // token is varName
        }
    }
}

// (expression (',' expression)*)?
int CompilationEngine_compileExpressionList(CompilationEngine thisObject)
{
    char symbol[JACK_TOKEN_SIZE];
    int num = 0;

    // expression is not ')'
    if (! isSymbolToken(thisObject, ")")) {
        CompilationEngine_compileExpression(thisObject);
        num++;

        while (isSymbolToken(thisObject, ",")) {
            // ','
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            // expression
            CompilationEngine_compileExpression(thisObject);
            num++;
        }
    }
    return num;
}

void getIdentifierKindString(SymbolTable symbolTable, char *token, char *category)
{
    switch (SymbolTable_kindOf(symbolTable, token))
    {
    case SYMBOL_TABLE_KIND_STATIC:
        strcpy(category, "static");
        break;
    case SYMBOL_TABLE_KIND_FIELD:
        strcpy(category, "field");
        break;
    case SYMBOL_TABLE_KIND_ARG:
        strcpy(category, "argument");
        break;
    case SYMBOL_TABLE_KIND_VAR:
        strcpy(category, "var");
        break;
    case SYMBOL_TABLE_KIND_NONE:
    default:
        strcpy(category, "none");
        break;
    }
}

void getKeywordString(JackTokenizer tokenizer, char *keyword)
{
    struct keyword {
        JackTokenizer_Keyword id;
        char *string;
    };
    struct keyword keywords[] = {
        { JACK_TOKENIZER_KEYWORD_CLASS,        "class" },
        { JACK_TOKENIZER_KEYWORD_METHOD,       "method" },
        { JACK_TOKENIZER_KEYWORD_FUNCTION,     "function" },
        { JACK_TOKENIZER_KEYWORD_CONSTRUCTION, "constructor" },
        { JACK_TOKENIZER_KEYWORD_INT,          "int" },
        { JACK_TOKENIZER_KEYWORD_BOOLEAN,      "boolean" },
        { JACK_TOKENIZER_KEYWORD_CHAR,         "char" },
        { JACK_TOKENIZER_KEYWORD_VOID,         "void" },
        { JACK_TOKENIZER_KEYWORD_VAR,          "var" },
        { JACK_TOKENIZER_KEYWORD_STATIC,       "static" },
        { JACK_TOKENIZER_KEYWORD_FIELD,        "field" },
        { JACK_TOKENIZER_KEYWORD_LET,          "let" },
        { JACK_TOKENIZER_KEYWORD_DO,           "do" },
        { JACK_TOKENIZER_KEYWORD_IF,           "if" },
        { JACK_TOKENIZER_KEYWORD_ELSE,         "else" },
        { JACK_TOKENIZER_KEYWORD_WHILE,        "while" },
        { JACK_TOKENIZER_KEYWORD_RETURN,       "return" },
        { JACK_TOKENIZER_KEYWORD_TRUE,         "true" },
        { JACK_TOKENIZER_KEYWORD_FALSE,        "false" },
        { JACK_TOKENIZER_KEYWORD_NULL,         "null" },
        { JACK_TOKENIZER_KEYWORD_THIS,         "this" },
    };
    JackTokenizer_Keyword id = JackTokenizer_keyword(tokenizer);

    for (size_t i = 0; i < sizeof(keywords) / sizeof(keywords[0]); i++) {
        if (id == keywords[i].id) {
            strcpy(keyword, keywords[i].string);
            break;
        }
    }
}

bool isKeywordToken(CompilationEngine thisObject, JackTokenizer_Keyword keyword)
{
    if (JackTokenizer_tokenType(thisObject->tokenizer) != JACK_TOKENIZER_TOKEN_TYPE_KEYWORD) {
        return false;
    }

    if (JackTokenizer_keyword(thisObject->tokenizer) != keyword) {
        return false;
    }

    return true;
}

bool isSymbolToken(CompilationEngine thisObject, char *symbol)
{
    char token[JACK_TOKEN_SIZE];

    if (JackTokenizer_tokenType(thisObject->tokenizer) != JACK_TOKENIZER_TOKEN_TYPE_SYMBOL) {
        return false;
    }

    JackTokenizer_symbol(thisObject->tokenizer, token);
    if (strcmp(token, symbol) != 0) {
        return false;
    }

    return true;
}

bool inSymbolListToken(CompilationEngine thisObject, ...)
{
    char token[JACK_TOKEN_SIZE];
    char* symbol;
    bool found = false;

    if (JackTokenizer_tokenType(thisObject->tokenizer) != JACK_TOKENIZER_TOKEN_TYPE_SYMBOL) {
        return found;
    }

    JackTokenizer_symbol(thisObject->tokenizer, token);

    va_list args;
    va_start(args, thisObject);

    while ((symbol = va_arg(args, char*)) != NULL) {
        if (strcmp(token, symbol) == 0) {
            found = true;
            break;
        }
    }

    va_end(args);

    return found;
}