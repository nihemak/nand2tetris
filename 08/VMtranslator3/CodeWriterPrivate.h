#ifndef _CODE_WRITER_PRIVATE_H_INCLUDE_
#define _CODE_WRITER_PRIVATE_H_INCLUDE_

#include <stdio.h>

void fputslist(FILE* fp, ...);

void writeArithmethicAdd(FILE* fpAsm);
void writeArithmethicSub(FILE* fpAsm);
void writeArithmethicNeg(FILE* fpAsm);
void writeArithmethicEq(FILE* fpAsm, char *skipLabel);
void writeArithmethicGt(FILE* fpAsm, char *skipLabel);
void writeArithmethicLt(FILE* fpAsm, char *skipLabel);
void writeArithmethicAnd(FILE* fpAsm);
void writeArithmethicOr(FILE* fpAsm);
void writeArithmethicNot(FILE* fpAsm);

void writePushConstant(FILE* fpAsm, int index);
void writePushLocal(FILE* fpAsm, int index);
void writePopLocal(FILE* fpAsm, int index);
void writePushArgument(FILE* fpAsm, int index);
void writePopArgument(FILE* fpAsm, int index);
void writePushThis(FILE* fpAsm, int index);
void writePopThis(FILE* fpAsm, int index);
void writePushThat(FILE* fpAsm, int index);
void writePopThat(FILE* fpAsm, int index);
void writePushPointer(FILE* fpAsm, int index);
void writePopPointer(FILE* fpAsm, int index);
void writePushTemp(FILE* fpAsm, int index);
void writePopTemp(FILE* fpAsm, int index);
void writePushStatic(FILE* fpAsm, char *vmFileName, int index);
void writePopStatic(FILE* fpAsm, char *vmFileName, int index);

#endif
