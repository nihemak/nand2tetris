#include "Parser.h"
#include "ParserPrivate.h"
#include <string.h>
#include <stdlib.h>

#define IF_CMP_RET(var, str, ret) if (strcmp(var, str) == 0) return ret

struct parser
{
    FILE* fpVm;
    char command[PARSER_COMMAND_MAX_LENGTH + 1];
    char arg1[PARSER_ARG1_MAX_LENGTH + 1];
    char arg2[PARSER_ARG2_MAX_LENGTH + 1];
};

Parser Parser_init(FILE *fpVm)
{
    static struct parser thisObject;

    thisObject.fpVm = fpVm;
    fseek(thisObject.fpVm, 0L, SEEK_SET);
    moveNextAdvance(thisObject.fpVm);
    strcpy(thisObject.command, "");
    strcpy(thisObject.arg1,    "");
    strcpy(thisObject.arg2,    "");

    return &thisObject;
}

bool Parser_hasMoreCommands(Parser thisObject)
{
    return ! isEndOfFile(thisObject->fpVm);
}

void Parser_advance(Parser thisObject)
{
    getToken(thisObject->fpVm, thisObject->command);

    switch (Parser_commandType(thisObject)) {
    case PARSER_COMMAND_TYPE_C_PUSH:
        skipSpaces(thisObject->fpVm);
        getToken(thisObject->fpVm, thisObject->arg1);
        skipSpaces(thisObject->fpVm);
        getToken(thisObject->fpVm, thisObject->arg2);
        break;
    default:
        strcpy(thisObject->arg1, "");
        strcpy(thisObject->arg2, "");
        break;
    }

    moveNextAdvance(thisObject->fpVm);
}

Parser_CommandType Parser_commandType(Parser thisObject)
{
    IF_CMP_RET(thisObject->command,  "add", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,  "sub", PARSER_COMMAND_TYPE_C_ARITHMETIC);   
    IF_CMP_RET(thisObject->command,  "neg", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,   "eq", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,   "gt", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,   "lt", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,  "and", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,   "or", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command,  "not", PARSER_COMMAND_TYPE_C_ARITHMETIC);
    IF_CMP_RET(thisObject->command, "push", PARSER_COMMAND_TYPE_C_PUSH);

    return -1;
}

void Parser_arg1(Parser thisObject, char *arg1)
{
    if (Parser_commandType(thisObject) == PARSER_COMMAND_TYPE_C_ARITHMETIC) {
        strcpy(arg1, thisObject->command);
    } else if (Parser_commandType(thisObject) == PARSER_COMMAND_TYPE_C_PUSH) {
        strcpy(arg1, thisObject->arg1);
    }
}

int Parser_arg2(Parser thisObject)
{
    if (Parser_commandType(thisObject) == PARSER_COMMAND_TYPE_C_PUSH) {
        return atoi(thisObject->arg2);
    }
    return -1;
}
