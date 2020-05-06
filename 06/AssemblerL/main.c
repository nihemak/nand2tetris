#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include "Parser.h"
#include "Code.h"

#define ASM_FILENAME_MAX_LENGTH (255)
// length(".hack") - length(".asm") + length('\0') = 2
#define HACK_FILENAME_MAX_LENGTH (ASM_FILENAME_MAX_LENGTH + 2)
// strlen(max(value)) = strlen(2^16 - 1) = strlen(65535) = 5
#define A_COMMAND_VALUE_LENGTH (15) 

// Assembler Prog.asm -> Prog.hack

bool getHackFileName(char *asmFileName, char *hackFileName);
void assemble(FILE *fpAsm, FILE *fpHack);
void assembleACommand(Parser parser, FILE *fpHack);
void assembleCCommand(Parser parser, FILE *fpHack);
void getACommandValueString(int valueNumber, char *valueString);

int main(int argc, char *argv[]) 
{
    char *asmFileName;
    char hackFileName[HACK_FILENAME_MAX_LENGTH];
    FILE *fpAsm, *fpHack;

    if (argc != 2) {
        fprintf(stderr, "Usage: Assembler [Prog].asm\n");
        return 1;
    }
    asmFileName = argv[1];

    if (strlen(asmFileName) > ASM_FILENAME_MAX_LENGTH) {
        fprintf(stderr, "Usage: Assembler [Prog].asm\n");
        fprintf(
            stderr, 
            "Asm filename max size is invalid. Max size is %d. (%s) is %lu\n", 
            ASM_FILENAME_MAX_LENGTH, 
            asmFileName,
            strlen(asmFileName)
        );
        return 1;
    }

    if (! getHackFileName(asmFileName, hackFileName)) {
        fprintf(stderr, "Usage: Assembler [Prog].asm\n");
        fprintf(stderr, "Asm filename extension(.asm) is invalid. (%s)\n", asmFileName);
        return 1;
    }

    if ((fpAsm = fopen(asmFileName, "r")) == NULL) {
        fprintf(stderr, "Error: asm file not found (%s)\n", asmFileName);
        return 1;
    }

    if ((fpHack = fopen(hackFileName, "w")) == NULL) {
        fprintf(stderr, "Error: hack file not open (%s)\n", hackFileName);
        fclose(fpAsm);
        return 1;
    }

    assemble(fpAsm, fpHack);

    fclose(fpHack);
    fclose(fpAsm);

    return 0;
}

bool getHackFileName(char *asmFileName, char *hackFileName)
{
    size_t asmFileNameLength  = strlen(asmFileName);
    size_t asmExtentionLength = strlen(".asm");

    if (strlen(asmFileName) <= asmExtentionLength) {
        return false;
    }

    // asm filename is Xxx.asm
    if (! (asmFileName[asmFileNameLength - 4] == '.' && 
           asmFileName[asmFileNameLength - 3] == 'a' &&
           asmFileName[asmFileNameLength - 2] == 's' &&
           asmFileName[asmFileNameLength - 1] == 'm')) {
        return false;
    }

    strncpy(hackFileName, asmFileName, asmFileNameLength - asmExtentionLength);
    strcat(hackFileName, ".hack");

    return true;
}

void assemble(FILE *fpAsm, FILE *fpHack)
{
    Parser parser = Parser_init(fpAsm);
    while (Parser_hasMoreCommands(parser)) {
        Parser_advance(parser);
        switch (Parser_commandType(parser)) {
        case PARSER_COMMAND_TYPE_A_COMMAND:
            assembleACommand(parser, fpHack);
            break;
        case PARSER_COMMAND_TYPE_C_COMMAND:
            assembleCCommand(parser, fpHack);
            break;
        default:
            break;
        }
    }
}

void assembleACommand(Parser parser, FILE *fpHack)
{
    char symbol[PARSER_SYMBOL_MAX_LENGTH + 1];
    int valueNumber;
    char value[A_COMMAND_VALUE_LENGTH + 1];

    Parser_symbol(parser, symbol);
    sscanf(symbol, "%d", &valueNumber);
    getACommandValueString(valueNumber, value);

    // 0 value
    fputs("0", fpHack);
    fputs(value, fpHack);
    fputs("\n", fpHack);
}

void assembleCCommand(Parser parser, FILE *fpHack)
{
    char mnemonic[PARSER_MNEMONIC_MAX_LENGTH + 1];
    char dest[PARSER_DEST_LENGTH + 1], comp[PARSER_COMP_LENGTH + 1], jump[PARSER_JUMP_LENGTH + 1];

    Parser_dest(parser, mnemonic);
    Code_dest(mnemonic, dest);

    Parser_comp(parser, mnemonic);
    Code_comp(mnemonic, comp);

    Parser_jump(parser, mnemonic);
    Code_jump(mnemonic, jump);

    // 111 comp dest jump
    fputs("111", fpHack);
    fputs(comp, fpHack);
    fputs(dest, fpHack);
    fputs(jump, fpHack);
    fputs("\n", fpHack);
}

void getACommandValueString(int valueNumber, char *valueString)
{
    sprintf(
        valueString,
        "%d%d%d%d%d%d%d%d%d%d%d%d%d%d%d",
        valueNumber & (int) pow(2, 14) ? 1 : 0,
        valueNumber & (int) pow(2, 13) ? 1 : 0,
        valueNumber & (int) pow(2, 12) ? 1 : 0,
        valueNumber & (int) pow(2, 11) ? 1 : 0,
        valueNumber & (int) pow(2, 10) ? 1 : 0,
        valueNumber & (int) pow(2,  9) ? 1 : 0,
        valueNumber & (int) pow(2,  8) ? 1 : 0,
        valueNumber & (int) pow(2,  7) ? 1 : 0,
        valueNumber & (int) pow(2,  6) ? 1 : 0,
        valueNumber & (int) pow(2,  5) ? 1 : 0,
        valueNumber & (int) pow(2,  4) ? 1 : 0,
        valueNumber & (int) pow(2,  3) ? 1 : 0,
        valueNumber & (int) pow(2,  2) ? 1 : 0,
        valueNumber & (int) pow(2,  1) ? 1 : 0,
        valueNumber & (int) pow(2,  0) ? 1 : 0
    );
}
