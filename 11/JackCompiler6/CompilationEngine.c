#include "CompilationEngine.h"
#include "JackTokenizer.h"
#include "SymbolTable.h"
#include "VMWriter.h"
#include "string.h"
#include <stdarg.h>

void getKeywordString(JackTokenizer tokenizer, char *keyword);
bool isKeywordToken(CompilationEngine thisObject, JackTokenizer_Keyword keyword);
bool isSymbolToken(CompilationEngine thisObject, char *symbol);
bool inSymbolListToken(CompilationEngine thisObject, ...);
VMWriter_Segment convertKindToSegment(SymbolTable_Kind kind);

typedef struct compilation_engine * CompilationEngine;
struct compilation_engine
{
    JackTokenizer tokenizer;
    SymbolTable symbolTable;
    WMWriter vmWriter;
    char className[JACK_TOKEN_SIZE];
    int whileLabelCount;
    int ifLabelCount;
};

CompilationEngine CompilationEngine_init(FILE *fpJack, FILE *fpVm)
{
    static struct compilation_engine thisObject;

    thisObject.tokenizer = JackTokenizer_init(fpJack);
    thisObject.symbolTable = SymbolTable_init();
    thisObject.vmWriter = WMWriter_init(fpVm);
    strcpy(thisObject.className, "");
    thisObject.whileLabelCount = 0;
    thisObject.ifLabelCount = 0;

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
    JackTokenizer_Keyword functionKind = JackTokenizer_keyword(thisObject->tokenizer);
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

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    if (functionKind == JACK_TOKENIZER_KEYWORD_METHOD) {
        // b.mult(5) => mult(b,5)
        // it is ok because "this" is keyword, not verName
        SymbolTable_define(thisObject->symbolTable, "this", thisObject->className, SYMBOL_TABLE_KIND_ARG);
    }

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
        VMWriter_writeFunction(
            thisObject->vmWriter,
            functionName,
            SymbolTable_varCount(thisObject->symbolTable, SYMBOL_TABLE_KIND_VAR)
        );
        if (functionKind == JACK_TOKENIZER_KEYWORD_CONSTRUCTION) {
            int fieldCount = SymbolTable_varCount(thisObject->symbolTable, SYMBOL_TABLE_KIND_FIELD);
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, fieldCount);
            VMWriter_writeCall(thisObject->vmWriter, "Memory.alloc", 1);
            VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 0);
        }
        if (functionKind == JACK_TOKENIZER_KEYWORD_METHOD) {
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_ARG, 0);
            VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 0);
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
    int nArgs = 0;

    // 'do'
    keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineName | (className | varName)
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, token);
    JackTokenizer_advance(thisObject->tokenizer);

    // '(' or '.'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    if (isSymbolToken(thisObject, ".")) {
        // (className | varName) used
        SymbolTable_Kind functionKind = SymbolTable_kindOf(thisObject->symbolTable, token);
        JackTokenizer_advance(thisObject->tokenizer);

        // subroutineName (subroutine used)
        JackTokenizer_identifier(thisObject->tokenizer, identifier);
        JackTokenizer_advance(thisObject->tokenizer);

        if (functionKind != SYMBOL_TABLE_KIND_NONE) {
            // token is varName
            char className[JACK_TOKEN_SIZE];
            SymbolTable_typeOf(thisObject->symbolTable, token, className);
            sprintf(functionName, "%s.%s", className, identifier);

            VMWriter_writePush(
                thisObject->vmWriter,
                convertKindToSegment(SymbolTable_kindOf(thisObject->symbolTable, token)),
                SymbolTable_indexOf(thisObject->symbolTable, token)
            );
            nArgs++;
        } else {
            // token is className
            sprintf(functionName, "%s.%s", token, identifier);
        }

        // '('
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    } else {
        // token is subroutineName (subroutine used)
        sprintf(functionName, "%s.%s", thisObject->className, token);

        VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 0);
        nArgs++;
    }
    JackTokenizer_advance(thisObject->tokenizer);

    nArgs += CompilationEngine_compileExpressionList(thisObject);

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
    char varName[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, varName);
    SymbolTable_Kind kind = SymbolTable_kindOf(thisObject->symbolTable, varName);
    JackTokenizer_advance(thisObject->tokenizer);

    // '[' or '='
    bool isArray = false;
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    if (isSymbolToken(thisObject, "[")) {
        // varName is Array
        isArray = true;
        JackTokenizer_advance(thisObject->tokenizer);

        // push index of array
        CompilationEngine_compileExpression(thisObject);
        // push varName
        VMWriter_writePush(
            thisObject->vmWriter,
            convertKindToSegment(kind),
            SymbolTable_indexOf(thisObject->symbolTable, varName)
        );
        // setup that segment 0 (1/2)
        VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_ADD);

        // ']'
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
        JackTokenizer_advance(thisObject->tokenizer);

        // '='
        JackTokenizer_symbol(thisObject->tokenizer, symbol);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    if (isArray) {
        // setup that segment 0 (2/2)
        // pop expression result, pop, push expression result
        VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_TEMP, 0);
        VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 1);
        VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_TEMP, 0);
        VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_THAT, 0);
    } else {
        VMWriter_writePop(
            thisObject->vmWriter,
            convertKindToSegment(kind),
            SymbolTable_indexOf(thisObject->symbolTable, varName)
        );
    }

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

    char labelExp[JACK_TOKEN_SIZE], labelEnd[JACK_TOKEN_SIZE];
    sprintf(labelExp, "%s$$$WHILE_EXP.%d", thisObject->className, thisObject->whileLabelCount);
    sprintf(labelEnd, "%s$$$WHILE_END.%d", thisObject->className, thisObject->whileLabelCount);
    thisObject->whileLabelCount++;

    VMWriter_writeLabel(thisObject->vmWriter, labelExp);

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_NOT);
    VMWriter_writeIf(thisObject->vmWriter, labelEnd);

    // '{'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    VMWriter_writeGoto(thisObject->vmWriter, labelExp);

    // '}'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writeLabel(thisObject->vmWriter, labelEnd);
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
    } else {
        VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, 0);
    }

    // ';'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

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

    char labelTrue[JACK_TOKEN_SIZE], labelFalse[JACK_TOKEN_SIZE], labelEnd[JACK_TOKEN_SIZE];
    sprintf(labelTrue, "%s$$$IF_TRUE.%d", thisObject->className, thisObject->ifLabelCount);
    sprintf(labelFalse, "%s$$$IF_FALSE.%d", thisObject->className, thisObject->ifLabelCount);
    sprintf(labelEnd, "%s$$$IF_END.%d", thisObject->className, thisObject->ifLabelCount);
    thisObject->ifLabelCount++;

    // '('
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writeIf(thisObject->vmWriter, labelTrue);
    VMWriter_writeGoto(thisObject->vmWriter, labelFalse);
    VMWriter_writeLabel(thisObject->vmWriter, labelTrue);

    // '{'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    // '}'
    JackTokenizer_symbol(thisObject->tokenizer, symbol);
    JackTokenizer_advance(thisObject->tokenizer);

    VMWriter_writeGoto(thisObject->vmWriter, labelEnd);
    VMWriter_writeLabel(thisObject->vmWriter, labelFalse);

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

    VMWriter_writeLabel(thisObject->vmWriter, labelEnd);
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
        if (strcmp(symbol, "-") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_SUB);
        }
        if (strcmp(symbol, "*") == 0) {
            VMWriter_writeCall(thisObject->vmWriter, "Math.multiply", 2);
        }
        if (strcmp(symbol, "/") == 0) {
            VMWriter_writeCall(thisObject->vmWriter, "Math.divide", 2);
        }
        if (strcmp(symbol, "&") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_AND);
        }
        if (strcmp(symbol, "|") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_OR);
        }
        if (strcmp(symbol, "<") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_LT);
        }
        if (strcmp(symbol, ">") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_GT);
        }
        if (strcmp(symbol, "=") == 0) {
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_EQ);
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

        int length = strlen(stringVal);
        VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, length);
        VMWriter_writeCall(thisObject->vmWriter, "String.new", 1);
        for (int i = 0; i < length; i++) {
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, stringVal[i]);
            VMWriter_writeCall(thisObject->vmWriter, "String.appendChar", 2);
        }
    } else if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD) {
        // keywordConstant
        keyword = JackTokenizer_keyword(thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        if (keyword == JACK_TOKENIZER_KEYWORD_TRUE) {
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, 0);
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_NOT);
        }
        if (keyword == JACK_TOKENIZER_KEYWORD_FALSE || keyword == JACK_TOKENIZER_KEYWORD_NULL) {
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_CONST, 0);
        }
        if (keyword == JACK_TOKENIZER_KEYWORD_THIS) {
            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 0);
        }
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

        VMWriter_writeArithmetic(
            thisObject->vmWriter,
            strcmp(symbol, "-") == 0 ? VM_WRITER_COMMAND_NEG : VM_WRITER_COMMAND_NOT
        );
    } else {    // varName | varName '[' expression ']' | subroutineCall
        // varName | subroutineName | className (used)
        char token[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, token);
        SymbolTable_Kind kind = SymbolTable_kindOf(thisObject->symbolTable, token);
        JackTokenizer_advance(thisObject->tokenizer);

        // '[' or '(' or '.' or not
        if (isSymbolToken(thisObject, "[")) {
            // token is Array of varName (varName[])
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            // push index of array
            CompilationEngine_compileExpression(thisObject);
            // push varName
            VMWriter_writePush(
                thisObject->vmWriter,
                convertKindToSegment(kind),
                SymbolTable_indexOf(thisObject->symbolTable, token)
            );
            // setup that segment 0
            VMWriter_writeArithmetic(thisObject->vmWriter, VM_WRITER_COMMAND_ADD);
            VMWriter_writePop(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 1);

            // ']'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_THAT, 0);
        } else if (inSymbolListToken(thisObject, "(", ".", NULL)) {
            char functionName[JACK_TOKEN_SIZE];
            int nArgs = 0;

            // '(' or '.'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            if (isSymbolToken(thisObject, ".")) {
                // token is (className | varName)
                JackTokenizer_advance(thisObject->tokenizer);

                // subroutineName (subroutine used)
                JackTokenizer_identifier(thisObject->tokenizer, identifier);
                JackTokenizer_advance(thisObject->tokenizer);

                if (kind != SYMBOL_TABLE_KIND_NONE) {
                    // token is varName
                    char className[JACK_TOKEN_SIZE];
                    SymbolTable_typeOf(thisObject->symbolTable, token, className);
                    sprintf(functionName, "%s.%s", className, identifier);

                    VMWriter_writePush(
                        thisObject->vmWriter,
                        convertKindToSegment(SymbolTable_kindOf(thisObject->symbolTable, token)),
                        SymbolTable_indexOf(thisObject->symbolTable, token)
                    );
                    nArgs++;
                } else {
                    // token is className
                    sprintf(functionName, "%s.%s", token, identifier);
                }

                // '('
                JackTokenizer_symbol(thisObject->tokenizer, symbol);
            } else {
                // token is subroutineName (subroutine used)
                sprintf(functionName, "%s.%s", thisObject->className, token);

                VMWriter_writePush(thisObject->vmWriter, VM_WRITER_SEGMENT_POINTER, 0);
                nArgs++;
            }
            JackTokenizer_advance(thisObject->tokenizer);

            nArgs += CompilationEngine_compileExpressionList(thisObject);

            // ')'
            JackTokenizer_symbol(thisObject->tokenizer, symbol);
            JackTokenizer_advance(thisObject->tokenizer);

            VMWriter_writeCall(thisObject->vmWriter, functionName, nArgs);
        } else {
            // token is varName
            VMWriter_writePush(
                thisObject->vmWriter,
                convertKindToSegment(kind),
                SymbolTable_indexOf(thisObject->symbolTable, token)
            );
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

VMWriter_Segment convertKindToSegment(SymbolTable_Kind kind)
{
    VMWriter_Segment segment;
    switch (kind)
    {
    case SYMBOL_TABLE_KIND_STATIC:
        segment = VM_WRITER_SEGMENT_STATIC;
        break;
    case SYMBOL_TABLE_KIND_VAR:
        segment = VM_WRITER_SEGMENT_LOCAL;
        break;
    case SYMBOL_TABLE_KIND_ARG:
        segment = VM_WRITER_SEGMENT_ARG;
        break;
    case SYMBOL_TABLE_KIND_FIELD:
    default:
        segment = VM_WRITER_SEGMENT_THIS;
        break;
    }
    return segment;
}
