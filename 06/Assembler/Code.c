#include "Code.h"
#include <string.h>

#define IF_CMP_CPY(condVar, condStr, destVar, destStr) if (strcmp(condVar, condStr) == 0) strcpy(destVar, destStr)

void Code_dest(char *mnemonic, char *dest)
{
    IF_CMP_CPY(mnemonic,    "", dest, "000");
    IF_CMP_CPY(mnemonic,   "M", dest, "001");
    IF_CMP_CPY(mnemonic,   "D", dest, "010");
    IF_CMP_CPY(mnemonic,  "MD", dest, "011");
    IF_CMP_CPY(mnemonic,   "A", dest, "100");
    IF_CMP_CPY(mnemonic,  "AM", dest, "101");
    IF_CMP_CPY(mnemonic,  "AD", dest, "110");
    IF_CMP_CPY(mnemonic, "AMD", dest, "111");
}

void Code_comp(char *mnemonic, char *comp)
{
    IF_CMP_CPY(mnemonic,   "0", comp, "0101010");
    IF_CMP_CPY(mnemonic,   "1", comp, "0111111");
    IF_CMP_CPY(mnemonic,  "-1", comp, "0111010");
    IF_CMP_CPY(mnemonic,   "D", comp, "0001100");
    IF_CMP_CPY(mnemonic,   "A", comp, "0110000"); IF_CMP_CPY(mnemonic,   "M", comp, "1110000");
    IF_CMP_CPY(mnemonic,  "!D", comp, "0001101");
    IF_CMP_CPY(mnemonic,  "!A", comp, "0110001"); IF_CMP_CPY(mnemonic,  "!M", comp, "1110001");
    IF_CMP_CPY(mnemonic,  "-D", comp, "0001111");
    IF_CMP_CPY(mnemonic,  "-A", comp, "0110011"); IF_CMP_CPY(mnemonic,  "-M", comp, "1110011");
    IF_CMP_CPY(mnemonic, "D+1", comp, "0011111");
    IF_CMP_CPY(mnemonic, "A+1", comp, "0110111"); IF_CMP_CPY(mnemonic, "M+1", comp, "1110111");
    IF_CMP_CPY(mnemonic, "D-1", comp, "0001110");
    IF_CMP_CPY(mnemonic, "A-1", comp, "0110010"); IF_CMP_CPY(mnemonic, "M-1", comp, "1110010");
    IF_CMP_CPY(mnemonic, "D+A", comp, "0000010"); IF_CMP_CPY(mnemonic, "D+M", comp, "1000010");
    IF_CMP_CPY(mnemonic, "D-A", comp, "0010011"); IF_CMP_CPY(mnemonic, "D-M", comp, "1010011");
    IF_CMP_CPY(mnemonic, "A-D", comp, "0000111"); IF_CMP_CPY(mnemonic, "M-D", comp, "1000111");
    IF_CMP_CPY(mnemonic, "D&A", comp, "0000000"); IF_CMP_CPY(mnemonic, "D&M", comp, "1000000");
    IF_CMP_CPY(mnemonic, "D|A", comp, "0010101"); IF_CMP_CPY(mnemonic, "D|M", comp, "1010101");
}

void Code_jump(char *mnemonic, char *jump)
{
    IF_CMP_CPY(mnemonic,    "", jump, "000");
    IF_CMP_CPY(mnemonic, "JGT", jump, "001");
    IF_CMP_CPY(mnemonic, "JEQ", jump, "010");
    IF_CMP_CPY(mnemonic, "JGE", jump, "011");
    IF_CMP_CPY(mnemonic, "JLT", jump, "100");
    IF_CMP_CPY(mnemonic, "JNE", jump, "101");
    IF_CMP_CPY(mnemonic, "JLE", jump, "110");
    IF_CMP_CPY(mnemonic, "JMP", jump, "111");
}
