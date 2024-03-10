#ifndef _JACK_TOKENIZER_H_INCLUDE_
#define _JACK_TOKENIZER_H_INCLUDE_

#include <stdio.h>
#include <stdbool.h>

#define JACK_TOKEN_SIZE 255

typedef enum {
    JACK_TOKENIZER_TOKEN_TYPE_KEYWORD = 1,
    JACK_TOKENIZER_TOKEN_TYPE_SYMBOL,
    JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER,
    JACK_TOKENIZER_TOKEN_TYPE_INT_CONST,
    JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST,
    JACK_TOKENIZER_TOKEN_TYPE_STRING_UNKNOWN
} JackTokenizer_TokenType;

typedef enum {
    JACK_TOKENIZER_KEYWORD_CLASS = 1,
    JACK_TOKENIZER_KEYWORD_METHOD,
    JACK_TOKENIZER_KEYWORD_FUNCTION,
    JACK_TOKENIZER_KEYWORD_CONSTRUCTION,
    JACK_TOKENIZER_KEYWORD_INT,
    JACK_TOKENIZER_KEYWORD_BOOLEAN,
    JACK_TOKENIZER_KEYWORD_CHAR,
    JACK_TOKENIZER_KEYWORD_VOID,
    JACK_TOKENIZER_KEYWORD_VAR,
    JACK_TOKENIZER_KEYWORD_STATIC,
    JACK_TOKENIZER_KEYWORD_FIELD,
    JACK_TOKENIZER_KEYWORD_LET,
    JACK_TOKENIZER_KEYWORD_DO,
    JACK_TOKENIZER_KEYWORD_IF,
    JACK_TOKENIZER_KEYWORD_ELSE,
    JACK_TOKENIZER_KEYWORD_WHILE,
    JACK_TOKENIZER_KEYWORD_RETURN,
    JACK_TOKENIZER_KEYWORD_TRUE,
    JACK_TOKENIZER_KEYWORD_FALSE,
    JACK_TOKENIZER_KEYWORD_NULL,
    JACK_TOKENIZER_KEYWORD_THIS,
    JACK_TOKENIZER_KEYWORD_UNKNOWN
} JackTokenizer_Keyword;

typedef struct jack_tokenizer * JackTokenizer;

JackTokenizer JackTokenizer_init(FILE *fpJack);
bool JackTokenizer_hasMoreTokens(JackTokenizer thisObject);
void JackTokenizer_advance(JackTokenizer thisObject);
JackTokenizer_TokenType JackTokenizer_tokenType(JackTokenizer thisObject);
JackTokenizer_Keyword JackTokenizer_keyword(JackTokenizer thisObject);
void JackTokenizer_symbol(JackTokenizer thisObject, char *symbol);
void JackTokenizer_identifier(JackTokenizer thisObject, char *identifier);
void JackTokenizer_intVal(JackTokenizer thisObject, int *intVal);
void JackTokenizer_stringVal(JackTokenizer thisObject, char *stringVal);

#endif
