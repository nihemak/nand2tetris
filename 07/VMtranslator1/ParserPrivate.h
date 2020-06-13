#ifndef _PARSER_PRIVATE_H_INCLUDE_
#define _PARSER_PRIVATE_H_INCLUDE_

#include <stdio.h>
#include <stdbool.h>

bool isSpace(FILE *fpVm);
bool isComment(FILE *fpVm);
bool isEndOfFile(FILE *fpVm);
bool isEndOfLine(FILE *fpVm);
bool isToken(FILE *fpVm);
void skipSpaces(FILE *fpVm);
void skipEndOFLines(FILE *fpVm);
void skipComment(FILE *fpVm);
void moveNextAdvance(FILE *fpVm);
void getToken(FILE *fpVm, char *token);

#endif
