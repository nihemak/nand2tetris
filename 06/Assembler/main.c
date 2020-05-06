#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <ctype.h>
#include "Parser.h"
#include "Code.h"
#include "SymbolTable.h"

#define ASM_FILENAME_MAX_LENGTH (255)
// length(".hack") - length(".asm") + length('\0') = 2
#define HACK_FILENAME_MAX_LENGTH (ASM_FILENAME_MAX_LENGTH + 2)
#define A_COMMAND_VALUE_LENGTH (15) 
#define VARIABLE_SYMBOL_RAM_ADDRESS_START (16)

// Assembler Prog.asm -> Prog.hack

bool getHackFileName(char *asmFileName, char *hackFileName);
void assemble(FILE *fpAsm, FILE *fpHack);
int assembleACommand(Parser parser, SymbolTable symbolTable, int ramAddress, FILE *fpHack);
void assembleCCommand(Parser parser, FILE *fpHack);
void setDefinedSymbol(SymbolTable symbolTable);
void setLabelSymbol(SymbolTable symbolTable, Parser parser);
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
    int ramAddress = VARIABLE_SYMBOL_RAM_ADDRESS_START;
    SymbolTable symbolTable = SymbolTable_init();

    setDefinedSymbol(symbolTable);
    setLabelSymbol(symbolTable, parser);

    parser = Parser_init(fpAsm);
    while (Parser_hasMoreCommands(parser)) {
        Parser_advance(parser);
        switch (Parser_commandType(parser)) {
        case PARSER_COMMAND_TYPE_A_COMMAND:
            ramAddress = assembleACommand(parser, symbolTable, ramAddress, fpHack);
            break;
        case PARSER_COMMAND_TYPE_C_COMMAND:
            assembleCCommand(parser, fpHack);
            break;
        default:
            break;
        }
    }

    SymbolTable_delete(symbolTable);
}

int assembleACommand(Parser parser, SymbolTable symbolTable, int ramAddress, FILE *fpHack)
{
    char symbol[PARSER_SYMBOL_MAX_LENGTH + 1];
    int valueNumber;
    char value[A_COMMAND_VALUE_LENGTH + 1];

    Parser_symbol(parser, symbol);
    if (! isdigit(symbol[0])) {  // symbol is /^[^\d].*/
        if (SymbolTable_contains(symbolTable, symbol)) {
            valueNumber = SymbolTable_getAddress(symbolTable, symbol);
        } else {
            SymbolTable_addEntry(symbolTable, symbol, ramAddress);
            valueNumber = ramAddress;
            ramAddress++;
        }
    } else {
        sscanf(symbol, "%d", &valueNumber);
    }
    getACommandValueString(valueNumber, value);

    // 0 value
    fputs("0", fpHack);
    fputs(value, fpHack);
    fputs("\n", fpHack);

    return ramAddress;
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

void setDefinedSymbol(SymbolTable symbolTable)
{
    // LABEL, RAM Address
    SymbolTable_addEntry(symbolTable, "SP",         0);
    SymbolTable_addEntry(symbolTable, "LCL",        1);
    SymbolTable_addEntry(symbolTable, "ARG",        2);
    SymbolTable_addEntry(symbolTable, "THIS",       3);
    SymbolTable_addEntry(symbolTable, "THAT",       4);
    SymbolTable_addEntry(symbolTable, "R0",         0);
    SymbolTable_addEntry(symbolTable, "R1",         1);
    SymbolTable_addEntry(symbolTable, "R2",         2);
    SymbolTable_addEntry(symbolTable, "R3",         3);
    SymbolTable_addEntry(symbolTable, "R4",         4);
    SymbolTable_addEntry(symbolTable, "R5",         5);
    SymbolTable_addEntry(symbolTable, "R6",         6);
    SymbolTable_addEntry(symbolTable, "R7",         7);
    SymbolTable_addEntry(symbolTable, "R8",         8);
    SymbolTable_addEntry(symbolTable, "R9",         9);
    SymbolTable_addEntry(symbolTable, "R10",       10);
    SymbolTable_addEntry(symbolTable, "R11",       11);
    SymbolTable_addEntry(symbolTable, "R12",       12);
    SymbolTable_addEntry(symbolTable, "R13",       13);
    SymbolTable_addEntry(symbolTable, "R14",       14);
    SymbolTable_addEntry(symbolTable, "R15",       15);
    SymbolTable_addEntry(symbolTable, "SCREEN", 16384);
    SymbolTable_addEntry(symbolTable, "KBD",    24576);
}

void setLabelSymbol(SymbolTable symbolTable, Parser parser)
{
    int romAddress = 0;
    char symbol[PARSER_SYMBOL_MAX_LENGTH + 1];

    while (Parser_hasMoreCommands(parser)) {
        Parser_advance(parser);
        switch (Parser_commandType(parser)) {
        case PARSER_COMMAND_TYPE_A_COMMAND:
        case PARSER_COMMAND_TYPE_C_COMMAND:
            romAddress++;
            break;
        case PARSER_COMMAND_TYPE_L_COMMAND:
            Parser_symbol(parser, symbol);
            SymbolTable_addEntry(symbolTable, symbol, romAddress);
            break;
        default:
            break;
        }
    }
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
