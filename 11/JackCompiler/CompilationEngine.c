#include "CompilationEngine.h"
#include "JackTokenizer.h"
#include "SymbolTable.h"
#include "string.h"
#include <stdarg.h>

void writeKeyword(FILE *fp, JackTokenizer tokenizer);
void writeSymbol(FILE *fp, JackTokenizer tokenizer);
void writeSymbolByToken(FILE *fp, char *token);
void writeIdentifier(FILE *fp, JackTokenizer tokenizer, char *category, char *status, SymbolTable symbolTable);
void getIdentifierKindString(SymbolTable symbolTable, char *token, char *category);
void writeIdentifierByToken(FILE *fp, char *token, char *category, char *status, SymbolTable symbolTable);
void writeIntegerConstant(FILE *fp, JackTokenizer tokenizer);
void writeStringConstant(FILE *fp, JackTokenizer tokenizer);
void getKeywordString(JackTokenizer tokenizer, char *keyword);
bool isKeywordToken(CompilationEngine thisObject, JackTokenizer_Keyword keyword);
bool isSymbolToken(CompilationEngine thisObject, char *symbol);
bool inSymbolListToken(CompilationEngine thisObject, ...);

typedef struct compilation_engine * CompilationEngine;
struct compilation_engine
{
    JackTokenizer tokenizer;
    SymbolTable symbolTable;
    FILE* fpXml;
};

CompilationEngine CompilationEngine_init(FILE *fpJack, FILE *fpXml)
{
    static struct compilation_engine thisObject;

    thisObject.tokenizer = JackTokenizer_init(fpJack);
    thisObject.symbolTable = SymbolTable_init();
    thisObject.fpXml = fpXml;

    return &thisObject;
}

// 'class' className '{' classVarDec* subroutineDec* '}'
void CompilationEngine_compileClass(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<class>\n");

    JackTokenizer_advance(thisObject->tokenizer);

    // 'class'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // className
    writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "defined", NULL);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
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
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</class>\n");

    SymbolTable_delete(thisObject->symbolTable);
    thisObject->symbolTable = SymbolTable_init();
}

// ('static' | 'field') type varName (',' varName)* ';'
void CompilationEngine_compileClassVarDec(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<classVarDec>\n");

    // ('static' | 'field')
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_Keyword keyword = JackTokenizer_keyword(thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer); 

    // type
    char type[JACK_TOKEN_SIZE];
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "used", NULL);
        JackTokenizer_identifier(thisObject->tokenizer, type);
    } else {
        writeKeyword(thisObject->fpXml, thisObject->tokenizer);
        getKeywordString(thisObject->tokenizer, type);
    }

    do {
        JackTokenizer_advance(thisObject->tokenizer);

        // varName
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(
            thisObject->symbolTable,
            varName,
            type,
            keyword == JACK_TOKENIZER_KEYWORD_STATIC ? SYMBOL_TABLE_KIND_STATIC : SYMBOL_TABLE_KIND_FIELD
        );
        writeIdentifier(
            thisObject->fpXml,
            thisObject->tokenizer,
            keyword == JACK_TOKENIZER_KEYWORD_STATIC ? "static" : "field",
            "defined",
            thisObject->symbolTable
        );
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or ';'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    } while (! isSymbolToken(thisObject, ";"));
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</classVarDec>\n");
}

// ('constructor' | 'function' | 'method') ('void' | type) subroutineName '(' parameterList ')' subroutineBody
// subroutineBody: '{' varDec* statements '}'
void CompilationEngine_compileSubroutine(CompilationEngine thisObject)
{
    SymbolTable_startSubroutine(thisObject->symbolTable);

    fprintf(thisObject->fpXml, "<subroutineDec>\n");

    // ('constructor' | 'function' | 'method')
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // ('void' | type)
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "used", NULL);
    } else {
        writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineName
    writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "subroutine", "defined", NULL);
    JackTokenizer_advance(thisObject->tokenizer);

    // '('
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileParameterList(thisObject);

    // ')'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "<subroutineBody>\n");

    // '{'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
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
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</subroutineBody>\n");

    fprintf(thisObject->fpXml, "</subroutineDec>\n");
}

// ((type varName) (',' type varName)*)?
void CompilationEngine_compileParameterList(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<parameterList>\n");

    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD || 
        JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        // type
        char type[JACK_TOKEN_SIZE];
        if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
            writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "used", NULL);
            JackTokenizer_identifier(thisObject->tokenizer, type);
        } else {
            writeKeyword(thisObject->fpXml, thisObject->tokenizer);
            getKeywordString(thisObject->tokenizer, type);
        }
        JackTokenizer_advance(thisObject->tokenizer);

        // varName
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_ARG);
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "argument", "defined", thisObject->symbolTable);
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or not
        while (isSymbolToken(thisObject, ",")) {
            // ','
            writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            JackTokenizer_advance(thisObject->tokenizer);

            // type
            if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
                writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "used", NULL);

                JackTokenizer_identifier(thisObject->tokenizer, type);
            } else {
                writeKeyword(thisObject->fpXml, thisObject->tokenizer);
                getKeywordString(thisObject->tokenizer, type);
            }
            JackTokenizer_advance(thisObject->tokenizer);

            // varName
            JackTokenizer_identifier(thisObject->tokenizer, varName);
            SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_ARG);
            writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "argument", "defined", thisObject->symbolTable);
            JackTokenizer_advance(thisObject->tokenizer);
        }
    }

    fprintf(thisObject->fpXml, "</parameterList>\n");
}

// 'var' type varName (',' varName)* ';'
void CompilationEngine_compileVarDec(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<varDec>\n");

    // 'var'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // type
    char type[JACK_TOKEN_SIZE];
    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "class", "used", NULL);
        JackTokenizer_identifier(thisObject->tokenizer, type);
    } else {
        writeKeyword(thisObject->fpXml, thisObject->tokenizer);
        getKeywordString(thisObject->tokenizer, type);
    }

    do {
        JackTokenizer_advance(thisObject->tokenizer);

        // varName
        char varName[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, varName);
        SymbolTable_define(thisObject->symbolTable, varName, type, SYMBOL_TABLE_KIND_VAR);
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "var", "defined", thisObject->symbolTable);
        JackTokenizer_advance(thisObject->tokenizer);

        // ',' or ';'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    } while (! isSymbolToken(thisObject, ";"));
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</varDec>\n");
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

    // 'do'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // subroutineName | (className | varName)
    char token1[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, token1);
    JackTokenizer_advance(thisObject->tokenizer);

    // '(' or '.'
    char token2[JACK_TOKEN_SIZE];
    JackTokenizer_symbol(thisObject->tokenizer, token2);
    if (isSymbolToken(thisObject, ".")) {
        // (className | varName)
        char category[JACK_TOKEN_SIZE];
        getIdentifierKindString(thisObject->symbolTable, token1, category);
        if (strcmp(category, "none") == 0) {
            strcpy(category, "class");
        }
        writeIdentifierByToken(
            thisObject->fpXml,
            token1,
            category,
            "used",
            strcmp(category, "class") != 0 ? thisObject->symbolTable : NULL
        );
        writeSymbolByToken(thisObject->fpXml, token2);
        JackTokenizer_advance(thisObject->tokenizer);

        // subroutineName
        writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "subroutine", "used", NULL);
        JackTokenizer_advance(thisObject->tokenizer);

        // '('
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    } else {
        writeIdentifierByToken(thisObject->fpXml, token1, "subroutine", "used", NULL);
        writeSymbolByToken(thisObject->fpXml, token2);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpressionList(thisObject);

    // ')'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // ';'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</doStatement>\n");
}

// 'let' varName ('[' expression ']')? '=' expression ';'
void CompilationEngine_compileLet(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<letStatement>\n");

    // 'let'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // varName
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(thisObject->tokenizer, token);
    char category[JACK_TOKEN_SIZE];
    getIdentifierKindString(thisObject->symbolTable, token, category);
    writeIdentifierByToken(thisObject->fpXml, token, category, "used", thisObject->symbolTable);
    JackTokenizer_advance(thisObject->tokenizer);

    // '[' or '='
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    if (isSymbolToken(thisObject, "[")) {
        JackTokenizer_advance(thisObject->tokenizer);
    
        CompilationEngine_compileExpression(thisObject);

        // ']'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        // '='
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    }
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ';'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</letStatement>\n");
}

// 'while' '(' expression ')' '{' statements '}'
void CompilationEngine_compileWhile(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<whileStatement>\n");

    // 'while'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '('
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    // '}'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</whileStatement>\n");
}

// 'return' expression? ';'
void CompilationEngine_compileReturn(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<returnStatement>\n");

    // 'return'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // expression or ';'
    if (! isSymbolToken(thisObject, ";")) {
        CompilationEngine_compileExpression(thisObject);
    }

    // ';'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    fprintf(thisObject->fpXml, "</returnStatement>\n");
}

// 'if' '(' expression ')' '{' statements '}' ('else' '{' statements '}')?
void CompilationEngine_compileIf(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<ifStatement>\n");

    // 'if'
    writeKeyword(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '('
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileExpression(thisObject);

    // ')'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // '{'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    CompilationEngine_compileStatements(thisObject);

    // '}'
    writeSymbol(thisObject->fpXml, thisObject->tokenizer);
    JackTokenizer_advance(thisObject->tokenizer);

    // 'else' or not
    if (isKeywordToken(thisObject, JACK_TOKENIZER_KEYWORD_ELSE)) {
        // 'else'
        writeKeyword(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        // '{'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileStatements(thisObject);

        // '}'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
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
        // op
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileTerm(thisObject);
    }

    fprintf(thisObject->fpXml, "</expression>\n");
}

// integerConstant | stringConstant | keywordConstant | varName | varName '[' expression ']' | subroutineCall | '(' expression ')' | unaryOp term
// subroutineCall: subroutineName '(' expressionList ')' | (className | varName) '.' subroutineName '(' expressionList ')'
// unaryOp: '-' | '~'
void CompilationEngine_compileTerm(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<term>\n");

    if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_INT_CONST) {
        // integerConstant
        writeIntegerConstant(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST) {
        // stringConstant
        writeStringConstant(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (JackTokenizer_tokenType(thisObject->tokenizer) == JACK_TOKENIZER_TOKEN_TYPE_KEYWORD) {
        // keywordConstant
        writeKeyword(thisObject->fpXml, thisObject->tokenizer); 
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (isSymbolToken(thisObject, "(")) {    // '(' expression ')'
        // '('
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileExpression(thisObject);

        // ')'
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);
    } else if (inSymbolListToken(thisObject, "-", "~", NULL)) { // unaryOp term
        // unaryOp
        writeSymbol(thisObject->fpXml, thisObject->tokenizer);
        JackTokenizer_advance(thisObject->tokenizer);

        CompilationEngine_compileTerm(thisObject);
    } else {    // varName | varName '[' expression ']' | subroutineCall
        // varName | subroutineName | className
        char token1[JACK_TOKEN_SIZE];
        JackTokenizer_identifier(thisObject->tokenizer, token1);
        char category[JACK_TOKEN_SIZE];
        getIdentifierKindString(thisObject->symbolTable, token1, category);
        if (strcmp(category, "none") == 0) {
            strcpy(category, "class");
        }
        // writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "subroutine", "used", NULL);
        JackTokenizer_advance(thisObject->tokenizer);

        // '[' or '(' or '.' or not
        if (isSymbolToken(thisObject, "[")) {
            // varName[]
            writeIdentifierByToken(thisObject->fpXml, token1, category, "used", thisObject->symbolTable);

            writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            JackTokenizer_advance(thisObject->tokenizer);

            CompilationEngine_compileExpression(thisObject);

            // ']'
            writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            JackTokenizer_advance(thisObject->tokenizer);
        } else if (inSymbolListToken(thisObject, "(", ".", NULL)) {
            if (isSymbolToken(thisObject, "(")) {
                // subroutineName
                writeIdentifierByToken(thisObject->fpXml, token1, "subroutine", "used", NULL);
            } else {    // "."
                // (className | varName)
                writeIdentifierByToken(
                    thisObject->fpXml,
                    token1,
                    category,
                    "used",
                    strcmp(category, "class") != 0 ? thisObject->symbolTable : NULL
                );
            }

            // '(' or '.'
            writeSymbol(thisObject->fpXml, thisObject->tokenizer);

            if (isSymbolToken(thisObject, ".")) {
                JackTokenizer_advance(thisObject->tokenizer);

                // subroutineName
                writeIdentifier(thisObject->fpXml, thisObject->tokenizer, "subroutine", "used", NULL);
                JackTokenizer_advance(thisObject->tokenizer);

                // '('
                writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            }
            JackTokenizer_advance(thisObject->tokenizer);

            CompilationEngine_compileExpressionList(thisObject);

            // ')'
            writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            JackTokenizer_advance(thisObject->tokenizer);
        } else {
            // varName
            writeIdentifierByToken(thisObject->fpXml, token1, category, "used", thisObject->symbolTable);
        }
    }

    fprintf(thisObject->fpXml, "</term>\n");
}

// (expression (',' expression)*)?
void CompilationEngine_compileExpressionList(CompilationEngine thisObject)
{
    fprintf(thisObject->fpXml, "<expressionList>\n");

    // expression is not ')'
    if (! isSymbolToken(thisObject, ")")) {
        CompilationEngine_compileExpression(thisObject);

        while (isSymbolToken(thisObject, ",")) {
            // ','
            writeSymbol(thisObject->fpXml, thisObject->tokenizer);
            JackTokenizer_advance(thisObject->tokenizer);

            // expression
            CompilationEngine_compileExpression(thisObject);
        }
    }

    fprintf(thisObject->fpXml, "</expressionList>\n");
}

void writeKeyword(FILE *fp, JackTokenizer tokenizer)
{
    char keyword[JACK_TOKEN_SIZE];
    getKeywordString(tokenizer, keyword);
    fprintf(fp, "<keyword> %s </keyword>\n", keyword);
}

void writeSymbol(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_symbol(tokenizer, token);
    writeSymbolByToken(fp, token);
}

void writeSymbolByToken(FILE *fp, char *token)
{
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

void writeIdentifier(FILE *fp, JackTokenizer tokenizer, char *category, char *status, SymbolTable symbolTable)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(tokenizer, token);
    writeIdentifierByToken(fp, token, category, status, symbolTable);
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

void writeIdentifierByToken(FILE *fp, char *token, char *category, char *status, SymbolTable symbolTable)
{
    fprintf(fp, "<identifier category=\"%s\" status=\"%s\"", category, status);
    if (symbolTable != NULL) {
        char kindStr[JACK_TOKEN_SIZE];
        getIdentifierKindString(symbolTable, token, kindStr);

        char typeStr[JACK_TOKEN_SIZE];
        SymbolTable_typeOf(symbolTable, token, typeStr);
        fprintf(fp, " kind=\"%s\" type=\"%s\" index=\"%d\"", kindStr, typeStr, SymbolTable_indexOf(symbolTable, token));
    }
    fprintf(fp, "> %s </identifier>\n", token);
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
