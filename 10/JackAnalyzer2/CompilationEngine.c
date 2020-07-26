#include "CompilationEngine.h"
#include "JackTokenizer.h"
#include "string.h"
#include <stdarg.h>

void writeToken(FILE *fp, JackTokenizer tokenizer);
void writeKeyword(FILE *fp, JackTokenizer tokenizer);
void writeSymbol(FILE *fp, JackTokenizer tokenizer);
void writeIdentifier(FILE *fp, JackTokenizer tokenizer);
void writeIntegerConstant(FILE *fp, JackTokenizer tokenizer);
void writeStringConstant(FILE *fp, JackTokenizer tokenizer);
void advanceAndWriteToken(CompilationEngine thisObject);
bool isKeywordToken(CompilationEngine thisObject, JackTokenizer_Keyword keyword);
bool isSymbolToken(CompilationEngine thisObject, char *symbol);
bool inSymbolListToken(CompilationEngine thisObject, ...);

typedef struct compilation_engine * CompilationEngine;
struct compilation_engine
{
    JackTokenizer tokenizer;
    FILE* fpXml;
};

CompilationEngine CompilationEngine_init(FILE *fpJack, FILE *fpXml)
{
    static struct compilation_engine thisObject;

    thisObject.tokenizer = JackTokenizer_init(fpJack);
    thisObject.fpXml = fpXml;

    return &thisObject;
}

// 'class' className '{' classVarDec* subroutineDec* '}'
void CompilationEngine_compileClass(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<class>\n");

    advanceAndWriteToken(thisObject);   // 'class'
    advanceAndWriteToken(thisObject);   // className
    advanceAndWriteToken(thisObject);   // '{'

    JackTokenizer_advance(thisObject->tokenizer);   // '}' or not
    while (! isSymbolToken(thisObject, "}")) {
        if (
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_STATIC) ||
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_FIELD)
        ) {  // classVarDec
            CompilationEngine_compileClassVarDec(thisObject);
        } else if (
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_CONSTRUCTION) ||
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_FUNCTION) ||
            isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_METHOD)
        ) {  // subroutineDec
            CompilationEngine_compileSubroutine(thisObject);
        } else {
            break;
        }
    }
    writeToken(thisObject->fpXml ,thisObject->tokenizer);    // '}'

    fprintf(thisObject->fpXml, "</class>\n");
}

// ('static' | 'field') type varName (',' varName)* ';'
void CompilationEngine_compileClassVarDec(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<classVarDec>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ('static' | 'field')
    advanceAndWriteToken(thisObject);   // type

    do {
        advanceAndWriteToken(thisObject);   // varName
        advanceAndWriteToken(thisObject);   // ',' or ';'
    } while (! isSymbolToken(thisObject, ";"));

    fprintf(thisObject->fpXml, "</classVarDec>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
// subroutineBody: '{' varDec* statements '}'
void CompilationEngine_compileSubroutine(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<subroutineDec>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ('constructor' | 'function' | 'method')
    advanceAndWriteToken(thisObject);   // ('void' | type)
    advanceAndWriteToken(thisObject);   // subroutineName
    advanceAndWriteToken(thisObject);   // '('

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileParameterList(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ')'

    fprintf(thisObject->fpXml, "<subroutineBody>\n");

    advanceAndWriteToken(thisObject);   // '{'

    JackTokenizer_advance(thisObject->tokenizer);   // 'var' or statements or '}'
    while (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_VAR)) {
        CompilationEngine_compileVarDec(thisObject);
    }
    if (! isSymbolToken(thisObject, "}")) {   // statements or '}'
        CompilationEngine_compileStatements(thisObject);
    }
    writeToken(thisObject->fpXml, thisObject->tokenizer);   // '}'

    fprintf(thisObject->fpXml, "</subroutineBody>\n");

    fprintf(thisObject->fpXml, "</subroutineDec>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// ((type varName) (',' type varName)*)?
void CompilationEngine_compileParameterList(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<parameterList>\n");

    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD) {  // type
        writeToken(thisObject->fpXml ,thisObject->tokenizer);   // type
        advanceAndWriteToken(thisObject);   // varName

        JackTokenizer_advance(thisObject->tokenizer);   // ',' or not
        while (isSymbolToken(thisObject, ",")) {
            writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ','
            advanceAndWriteToken(thisObject);   // type
            advanceAndWriteToken(thisObject);   // varName
            JackTokenizer_advance(thisObject->tokenizer);   // ',' or not
        }
    }

    fprintf(thisObject->fpXml, "</parameterList>\n");
}

// 'var' type varName (',' varName)* ';'
void CompilationEngine_compileVarDec(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<varDec>\n");

    writeToken(thisObject->fpXml, thisObject->tokenizer);   // 'var'
    advanceAndWriteToken(thisObject);   // type

    do {
        advanceAndWriteToken(thisObject);   // varName
        advanceAndWriteToken(thisObject);   // ',' or ';'
    } while (! isSymbolToken(thisObject, ";"));

    fprintf(thisObject->fpXml, "</varDec>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// statement*
// statement: letStatement | ifStatement | whileStatement | doStatement | returnStatement
void CompilationEngine_compileStatements(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<statements>\n");

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

    fprintf(thisObject->fpXml, "</statements>\n");
}

// 'do' subroutineCall ';'
// subroutineCall: subroutineName '(' expressionList ')' | (className | varName) '.' subroutineName '(' expressionList ')'
void CompilationEngine_compileDo(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<doStatement>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'do'

    advanceAndWriteToken(thisObject);   // subroutineName | (className | varName)
    advanceAndWriteToken(thisObject);   // '(' or '.'
    if (isSymbolToken(thisObject, ".")) {
        advanceAndWriteToken(thisObject);   // subroutineName
        advanceAndWriteToken(thisObject);   // '('
    }

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileExpressionList(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ')'
    advanceAndWriteToken(thisObject);   // ';'

    fprintf(thisObject->fpXml, "</doStatement>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// 'let' varName ('[' expression ']')? '=' expression ';'
void CompilationEngine_compileLet(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<letStatement>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'let'
    advanceAndWriteToken(thisObject);   // varName
    advanceAndWriteToken(thisObject);   // '[' or '='
    if (isSymbolToken(thisObject, "[")) {
        JackTokenizer_advance(thisObject->tokenizer);
        CompilationEngine_compileExpression(thisObject);

        writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ']'
        advanceAndWriteToken(thisObject);   // '='
    }

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileExpression(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ';'

    fprintf(thisObject->fpXml, "</letStatement>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// 'while' '(' expression ')' '{' statements '}'
void CompilationEngine_compileWhile(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<whileStatement>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'while'
    advanceAndWriteToken(thisObject);   // '('

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileExpression(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ')'
    advanceAndWriteToken(thisObject);   // '{'

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileStatements(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // '}'

    fprintf(thisObject->fpXml, "</whileStatement>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// 'return' expression? ';'
void CompilationEngine_compileReturn(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<returnStatement>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'return'

    JackTokenizer_advance(thisObject->tokenizer);   // expression or ';'
    if (! isSymbolToken(thisObject, ";")) {
        CompilationEngine_compileExpression(thisObject);
    }
    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ';'

    fprintf(thisObject->fpXml, "</returnStatement>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// 'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?
void CompilationEngine_compileIf(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<ifStatement>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'if'
    advanceAndWriteToken(thisObject);   // '('

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileExpression(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ')'
    advanceAndWriteToken(thisObject);   // '{'

    JackTokenizer_advance(thisObject->tokenizer);
    CompilationEngine_compileStatements(thisObject);

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // '}'

    JackTokenizer_advance(thisObject->tokenizer);   // 'else' or not
    if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_ELSE)) {
        writeToken(thisObject->fpXml ,thisObject->tokenizer);   // 'else'
        advanceAndWriteToken(thisObject);   // '{'

        JackTokenizer_advance(thisObject->tokenizer);
        CompilationEngine_compileStatements(thisObject);

        writeToken(thisObject->fpXml ,thisObject->tokenizer);   // '}'

        JackTokenizer_advance(thisObject->tokenizer);
    }

    fprintf(thisObject->fpXml, "</ifStatement>\n");
}

// term (op term)*
// op: '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='
void CompilationEngine_compileExpression(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<expression>\n");

    CompilationEngine_compileTerm(thisObject);

    while (inSymbolListToken(thisObject, "+", "-", "*",  "/", "&", "|", "<", ">", "=", NULL)) {
        writeToken(thisObject->fpXml ,thisObject->tokenizer);   // op

        JackTokenizer_advance(thisObject->tokenizer);
        CompilationEngine_compileTerm(thisObject);
    }

    fprintf(thisObject->fpXml, "</expression>\n");
}

// integerConstant | stringConstant | keywordConstant | varName | varName '[' expression ']' | subroutineCall | '(' expression ')' | unaryOp term
void CompilationEngine_compileTerm(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<term>\n");

    writeToken(thisObject->fpXml ,thisObject->tokenizer);   // term

    fprintf(thisObject->fpXml, "</term>\n");

    JackTokenizer_advance(thisObject->tokenizer);
}

// (expression (',' expression)*)?
void CompilationEngine_compileExpressionList(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<expressionList>\n");

    if (! isSymbolToken(thisObject, ")")) { // expression is not ')'
        CompilationEngine_compileExpression(thisObject);

        while (isSymbolToken(thisObject, ",")) {
            writeToken(thisObject->fpXml ,thisObject->tokenizer);   // ','

            JackTokenizer_advance(thisObject->tokenizer);   // expression
            CompilationEngine_compileExpression(thisObject);
        }
    }

    fprintf(thisObject->fpXml, "</expressionList>\n");
}

void writeToken(FILE *fp, JackTokenizer tokenizer)
{
    switch (JackTokenizer_tokenType(tokenizer))
    {
    case JACK_TOKENIZER_TOKEN_TYPE_KEYWORD:
        writeKeyword(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_SYMBOL:
        writeSymbol(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER:
        writeIdentifier(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_INT_CONST:
        writeIntegerConstant(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST:
        writeStringConstant(fp, tokenizer);
        break; 
    default:
        break;
    }
}

void writeKeyword(FILE *fp, JackTokenizer tokenizer)
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

    fprintf(fp, "<keyword> ");
    for (size_t i = 0; i < sizeof(keywords) / sizeof(keywords[0]); i++) {
        if (id == keywords[i].id) {
            fprintf(fp, "%s", keywords[i].string);
            break;
        }
    }
    fprintf(fp, " </keyword>\n");
}

void writeSymbol(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_symbol(tokenizer, token);

    fprintf(fp, "<symbol> ");
    if (strcmp(token, "<") == 0) {
        fprintf(fp, "&lt;");
    } else if (strcmp(token, ">") == 0) {
        fprintf(fp, "&gt;");
    } else if (strcmp(token, "&") == 0) {
        fprintf(fp, "&amp;");
    } else {
        fprintf(fp, "%s", token);
    }
    fprintf(fp, " </symbol>\n");
}

void writeIdentifier(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(tokenizer, token);
    fprintf(fp, "<identifier> %s </identifier>\n", token);
}

void writeIntegerConstant(FILE *fp, JackTokenizer tokenizer)
{
    int intVal;
    JackTokenizer_intVal(tokenizer, &intVal);
    fprintf(fp, "<integerConstant> %d </integerConstant>\n", intVal);
}

void writeStringConstant(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_stringVal(tokenizer, token);
    fprintf(fp, "<stringConstant> %s </stringConstant>\n", token);
}

void advanceAndWriteToken(CompilationEngine thisObject)
{
    JackTokenizer_advance(thisObject->tokenizer);
    writeToken(thisObject->fpXml ,thisObject->tokenizer);
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
