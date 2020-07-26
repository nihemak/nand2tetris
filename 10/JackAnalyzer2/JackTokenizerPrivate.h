#ifndef _JACK_TOKENIZER_PRIVATE_H_INCLUDE_
#define _JACK_TOKENIZER_PRIVATE_H_INCLUDE_

#include <stdio.h>
#include <stdbool.h>

void moveNextToken(FILE *fp);
bool isEndOfFile(FILE *fp);
bool getTokenSymbol(FILE *fp, char *token);
bool getTokenStringConstant(FILE *fp, char *token);
bool getTokenIntConstant(FILE *fp, char *token);
bool getTokenIdentifierOrKeyword(FILE *fp, char *token);

#endif
