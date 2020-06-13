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

#endif
