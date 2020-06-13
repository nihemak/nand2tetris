#include "CodeWriterPrivate.h"
#include <stdarg.h>

#define PUSH_CONSTANT_INDEX_MAX_DIGIT (6)

void writeArithmethicUnaryOperation(FILE* fpAsm, char *comp);
void writeArithmethicBinaryOperation(FILE* fpAsm, char *comp);
void writeArithmethicCondition(FILE* fpAsm, char *skipLabel, char *jump);

// pop y, pop x, push (x + y)
void writeArithmethicAdd(FILE* fpAsm) { writeArithmethicBinaryOperation(fpAsm, "D+M"); }
// pop y, pop x, push (x - y)
void writeArithmethicSub(FILE* fpAsm) { writeArithmethicBinaryOperation(fpAsm, "M-D"); }
// pop y, push (-y)
void writeArithmethicNeg(FILE* fpAsm) { writeArithmethicUnaryOperation(fpAsm, "-M"); }

// pop y, pop x, push (x == y ? -1(true/0xFFFF) : 0(false/0x0000))
void writeArithmethicEq(FILE* fpAsm, char *skipLabel) { writeArithmethicCondition(fpAsm, skipLabel, "JEQ"); }
// pop y, pop x, push (x >  y ? -1(true/0xFFFF) : 0(false/0x0000))
void writeArithmethicGt(FILE* fpAsm, char *skipLabel) { writeArithmethicCondition(fpAsm, skipLabel, "JGT"); }
// pop y, pop x, push (x <  y ? -1(true/0xFFFF) : 0(false/0x0000))
void writeArithmethicLt(FILE* fpAsm, char *skipLabel) { writeArithmethicCondition(fpAsm, skipLabel, "JLT"); }

// pop y, pop x, push (x and y)
void writeArithmethicAnd(FILE* fpAsm) { writeArithmethicBinaryOperation(fpAsm, "D&M"); }
// pop y, pop x, push (x or y)
void writeArithmethicOr(FILE* fpAsm)  { writeArithmethicBinaryOperation(fpAsm, "D|M"); }
// pop y, push (not y)
void writeArithmethicNot(FILE* fpAsm) { writeArithmethicUnaryOperation(fpAsm, "!M"); }

// push index
void writePushConstant(FILE* fpAsm, int index)
{
    char indexStr[PUSH_CONSTANT_INDEX_MAX_DIGIT + 1];
    sprintf(indexStr, "%d", index);

    fputslist(
        fpAsm,
        "// push constant\n",
        // Memory[Memory[SP]] <- index
        "@", indexStr, "\n",
        "D=A\n",
        "@SP\n",
        "A=M\n",
        "M=D\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// Unary operation (M <- y)
void writeArithmethicUnaryOperation(FILE* fpAsm, char *comp)
{
    fputslist(
        fpAsm,
        "// UnaryOperation ", comp, "\n",
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // y <- Memory[Memory[SP]]
        "A=M\n",
        // Memory[Memory[SP]] <- comp
        "M=", comp, "\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// Binary operation (M <- x, D <- y)
void writeArithmethicBinaryOperation(FILE* fpAsm, char *comp)
{
    fputslist(
        fpAsm,
        "// BinaryOperation ", comp, "\n",
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // y <- Memory[Memory[SP]]
        "A=M\n",
        "D=M\n",
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // x <- Memory[Memory[SP]]
        "A=M\n",
        // Memory[Memory[SP]] <- comp
        "M=", comp, "\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// Condition (comp <- x - y)
void writeArithmethicCondition(FILE* fpAsm, char *skipLabel, char *jump)
{
    fputslist(
        fpAsm,
        "// Condition ", jump, "\n",
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // y <- Memory[Memory[SP]]
        "A=M\n",
        "D=M\n",
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // x <- Memory[Memory[SP]]
        "A=M\n",
        // Register <- x - y
        "D=M-D\n",
        // Memory[Memory[SP]] <- -1(true/0xFFFF)
        "@SP\n",
        "A=M\n",
        "M=-1\n",
        // if jump(x - y) then goto skipLabel
        "@", skipLabel, "\n",
        "D;", jump, "\n",
        // Memory[Memory[SP] <- 0(false/0x0000)
        "@SP\n",
        "A=M\n",
        "M=0\n",
        // skipLabel:
        "(", skipLabel, ")\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

void fputslist(FILE* fp, ...)
{
    char* string;

    va_list args;
    va_start(args, fp);

    while ((string = va_arg(args, char*)) != NULL) {
        fputs(string, fp);
    }

    va_end(args);
}
