#ifndef _PARSER_H_INCLUDE_
#define _PARSER_H_INCLUDE_

#include <stdio.h>
#include <stdbool.h>

#define PARSER_COMMAND_MAX_LENGTH (4)
#define PARSER_ARG1_MAX_LENGTH    (8)
#define PARSER_ARG2_MAX_LENGTH    (16)

typedef enum {
    PARSER_COMMAND_TYPE_C_ARITHMETIC = 1,
    PARSER_COMMAND_TYPE_C_PUSH,
    PARSER_COMMAND_TYPE_C_POP
} Parser_CommandType;

typedef struct parser * Parser;

Parser Parser_init(FILE *fpVm);
bool Parser_hasMoreCommands(Parser thisObject);
void Parser_advance(Parser thisObject);
Parser_CommandType Parser_commandType(Parser thisObject);
void Parser_arg1(Parser thisObject, char *arg1);
int Parser_arg2(Parser thisObject);

#endif
