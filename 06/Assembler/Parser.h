#ifndef _PARSER_H_INCLUDE_
#define _PARSER_H_INCLUDE_

#include <stdio.h>
#include <stdbool.h>

#define PARSER_MNEMONIC_MAX_LENGTH (3)
#define PARSER_DEST_LENGTH         (3)
#define PARSER_COMP_LENGTH         (7)
#define PARSER_JUMP_LENGTH         (3)
#define PARSER_SYMBOL_MAX_LENGTH   (255)

typedef enum {
    PARSER_COMMAND_TYPE_A_COMMAND = 1,
    PARSER_COMMAND_TYPE_C_COMMAND,
    PARSER_COMMAND_TYPE_L_COMMAND
} Parser_CommandType;

typedef struct parser * Parser;

Parser Parser_init(FILE *fpAsm);
bool Parser_hasMoreCommands(Parser thisObject);
void Parser_advance(Parser thisObject);
Parser_CommandType Parser_commandType(Parser thisObject);
void Parser_symbol(Parser thisObject, char *symbol);
void Parser_dest(Parser thisObject, char *dest);
void Parser_comp(Parser thisObject, char *comp);
void Parser_jump(Parser thisObject, char *jump);

#endif
