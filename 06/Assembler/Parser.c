#include "Parser.h"
#include <string.h>

// c_command is dest=comp;jump
#define PARSER_C_COMMAND_MAX_LENGTH (PARSER_DEST_LENGTH + PARSER_COMP_LENGTH + PARSER_JUMP_LENGTH + 2)
// (symbol)
#define PARSER_ADVANCE_MAX_LENGTH   (PARSER_SYMBOL_MAX_LENGTH + 2)

struct parser
{
    FILE* fpAsm;
    char advance[PARSER_ADVANCE_MAX_LENGTH + 1];
};

bool isSpace(FILE *fpAsm);
bool isComment(FILE *fpAsm);
bool isEndOfFile(FILE *fpAsm);
bool isEndOfLine(FILE *fpAsm);
bool isAdvance(FILE *fpAsm);
void skipSpaces(FILE *fpAsm);
void skipEndOFLines(FILE *fpAsm);
void skipComment(FILE *fpAsm);
void moveNextAdvance(FILE *fpAsm);

Parser Parser_init(FILE *fpAsm)
{
    static struct parser thisObject;

    thisObject.fpAsm = fpAsm;
    fseek(thisObject.fpAsm, 0L, SEEK_SET);
    moveNextAdvance(thisObject.fpAsm);
    strcpy(thisObject.advance, "");

    return &thisObject;
}

bool Parser_hasMoreCommands(Parser thisObject)
{
    return ! isEndOfFile(thisObject->fpAsm);
}

void Parser_advance(Parser thisObject)
{
    int i = 0;
    while (isAdvance(thisObject->fpAsm)) {
        int c = fgetc(thisObject->fpAsm);
        thisObject->advance[i] = (char)c;
        i++;
    }
    thisObject->advance[i] = '\0';
    moveNextAdvance(thisObject->fpAsm);
}

Parser_CommandType Parser_commandType(Parser thisObject)
{
    if (thisObject->advance[0] == '@') {  // @Xxx
        return PARSER_COMMAND_TYPE_A_COMMAND;
    }
    if (thisObject->advance[0] == '(' &&
        thisObject->advance[strlen(thisObject->advance) - 1] == ')') {  // (Xxx)
        return PARSER_COMMAND_TYPE_L_COMMAND;
    }
    return PARSER_COMMAND_TYPE_C_COMMAND;
}

// advance must be A_COMMAND or L_COMMAND
void Parser_symbol(Parser thisObject, char *symbol)
{
    int i = 1;  // index 0 is '@' or '('. '@' and '(' are unnecessary.
    int advanceLength = strlen(thisObject->advance);
    if (Parser_commandType(thisObject) == PARSER_COMMAND_TYPE_L_COMMAND) {
        advanceLength--;  // index -1 is ')'. ')' is unnecessary.
    }
    for (; i < advanceLength; i++) {
        symbol[i - 1] = thisObject->advance[i];
    }
    symbol[i - 1] = '\0';
}

// advance must be C_COMMAND
void Parser_dest(Parser thisObject, char *dest)
{
    // advance is dest=comp;jump or dest=comp
    // dest is /^(?<dest>.+)=/

    bool isDest = true;
    int j = 0;
    char destCopy[PARSER_C_COMMAND_MAX_LENGTH + 1];
    for (size_t i = 0; i < strlen(thisObject->advance); i++) {
        if (isDest && thisObject->advance[i] == '=') {
            isDest = false;
            break;
        }
        destCopy[j] = thisObject->advance[i];
        j++;
    }
    destCopy[j] = '\0';

    if (! isDest) {
        strcpy(dest, destCopy);
    } else {
        dest[0] = '\0';
    }
}

void Parser_comp(Parser thisObject, char *comp)
{
    // advance is dest=comp;jump or dest=comp or comp;jump
    // comp is /^(?:.+=)?(?<comp>.+)(?:;.+)?$/ $1

    int j = 0;
    for (size_t i = 0; i < strlen(thisObject->advance); i++) {
        if (thisObject->advance[i] == '=') {
            j = 0;
            continue;
        }
        if (thisObject->advance[i] == ';') {
            break;
        }
        comp[j] = thisObject->advance[i];
        j++;
    }
    comp[j] = '\0';
}

void Parser_jump(Parser thisObject, char *jump)
{
    // advance is dest=comp;jump or comp;jump
    // jump is /;(?<jump>.+)$/

    bool isJump = false;
    int j = 0;
    for (size_t i = 0; i < strlen(thisObject->advance); i++)
    {
        if (! isJump && thisObject->advance[i] == ';') {
            isJump = true;
            continue;
        }
        if (! isJump) {
            continue;
        }
        jump[j] = thisObject->advance[i];
        j++;
    }
    jump[j] = '\0';
}

bool isSpace(FILE *fpAsm)
{
    int c = fgetc(fpAsm);
    ungetc(c, fpAsm);

    if ((char)c == ' ' || (char)c == '\t') {
        return true;
    }
    return false;
}

void skipSpaces(FILE *fpAsm)
{
    while (isSpace(fpAsm)) {
        fgetc(fpAsm);
    }
}

bool isComment(FILE *fpAsm)
{
    int c1 = fgetc(fpAsm);
    if ((char)c1 != '/') {
        ungetc(c1, fpAsm);
        return false;
    }
    int c2 = fgetc(fpAsm);
    ungetc(c2, fpAsm);
    ungetc(c1, fpAsm);
    if ((char)c2 == '/') {
        return true;
    }
    return false;
}

bool isEndOfFile(FILE *fpAsm)
{
    int c = fgetc(fpAsm);
    ungetc(c, fpAsm);

    if (c == EOF) {
        return true;
    }
    return false;
}

bool isEndOfLine(FILE *fpAsm)
{
    int c = fgetc(fpAsm);
    ungetc(c, fpAsm);
   
    if ((char)c == '\n' || (char)c == '\r') {
        return true;
    }
    return false;
}

bool isAdvance(FILE *fpAsm)
{
    return ! (isSpace(fpAsm) || isEndOfFile(fpAsm) || isEndOfLine(fpAsm) || isComment(fpAsm));
}

void skipEndOFLines(FILE *fpAsm)
{
    while (isEndOfLine(fpAsm)) {
        fgetc(fpAsm);
    }
}

void skipComment(FILE *fpAsm)
{
    if (isComment(fpAsm)) {
        do {
            fgetc(fpAsm);
        }
        while (! (isEndOfLine(fpAsm) || isEndOfFile(fpAsm)));
    }
}

void moveNextAdvance(FILE *fpAsm)
{
    do {
        skipEndOFLines(fpAsm);
        skipSpaces(fpAsm);
        skipComment(fpAsm);
    }
    while (isEndOfLine(fpAsm));
}
