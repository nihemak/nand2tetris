#include "JackTokenizer.h"
#include "JackTokenizerPrivate.h"
#include <stdlib.h>
#include <string.h>

typedef struct jack_tokenizer * JackTokenizer;
struct jack_tokenizer
{
    FILE* fpJack;
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_TokenType type;
    JackTokenizer_Keyword keyword;
};

JackTokenizer_Keyword convertIdentifierToKeyword(char *token);

JackTokenizer JackTokenizer_init(FILE *fpJack)
{
    static struct jack_tokenizer thisObject;

    thisObject.fpJack = fpJack;
    fseek(thisObject.fpJack, 0L, SEEK_SET);
    moveNextToken(thisObject.fpJack);
    strcpy(thisObject.token, "");
    thisObject.type = JACK_TOKENIZER_TOKEN_TYPE_STRING_UNKNOWN;
    thisObject.keyword = JACK_TOKENIZER_KEYWORD_UNKNOWN;

    return &thisObject;
}

bool JackTokenizer_hasMoreTokens(JackTokenizer thisObject)
{
    return ! isEndOfFile(thisObject->fpJack);
}

void JackTokenizer_advance(JackTokenizer thisObject)
{
    thisObject->keyword = JACK_TOKENIZER_KEYWORD_UNKNOWN;
    if (getTokenSymbol(thisObject->fpJack, thisObject->token)) {
        thisObject->type = JACK_TOKENIZER_TOKEN_TYPE_SYMBOL;
    } else if (getTokenStringConstant(thisObject->fpJack, thisObject->token)) {
        thisObject->type = JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST;
    } else if (getTokenIntConstant(thisObject->fpJack, thisObject->token)) {
        thisObject->type = JACK_TOKENIZER_TOKEN_TYPE_INT_CONST;
    } else {
        getTokenIdentifierOrKeyword(thisObject->fpJack, thisObject->token);

        thisObject->keyword = convertIdentifierToKeyword(thisObject->token);
        if (thisObject->keyword == JACK_TOKENIZER_KEYWORD_UNKNOWN) {
            thisObject->type = JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER;
        } else {
            thisObject->type = JACK_TOKENIZER_TOKEN_TYPE_KEYWORD;
        }
    }
    moveNextToken(thisObject->fpJack);
}

JackTokenizer_TokenType JackTokenizer_tokenType(JackTokenizer thisObject)
{
    return thisObject->type;
}

JackTokenizer_Keyword JackTokenizer_keyword(JackTokenizer thisObject)
{
    return thisObject->keyword;
}

void JackTokenizer_symbol(JackTokenizer thisObject, char *symbol)
{
    if (thisObject->type == JACK_TOKENIZER_TOKEN_TYPE_SYMBOL) {
        strcpy(symbol, thisObject->token);
    }
}

void JackTokenizer_identifier(JackTokenizer thisObject, char *identifier)
{
    if (thisObject->type == JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER) {
        strcpy(identifier, thisObject->token);
    }
}

void JackTokenizer_intVal(JackTokenizer thisObject, int *intVal)
{
    if (thisObject->type == JACK_TOKENIZER_TOKEN_TYPE_INT_CONST) {
        *intVal = atoi(thisObject->token);
    }
}

void JackTokenizer_stringVal(JackTokenizer thisObject, char *stringVal)
{
    if (thisObject->type == JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST) {
        strcpy(stringVal, thisObject->token);
    }
}

JackTokenizer_Keyword convertIdentifierToKeyword(char *token)
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
    for (size_t i = 0; i < sizeof(keywords) / sizeof(keywords[0]); i++) {
        if (strcmp(token, keywords[i].string) == 0) {
            return keywords[i].id;
        }
    }
    return JACK_TOKENIZER_KEYWORD_UNKNOWN;
}
