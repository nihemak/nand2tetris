#include "CodeWriterPrivate.h"
#include "CodeWriter.h"
#include <stdarg.h>

#define PUSH_POP_INDEX_MAX_DIGIT   (6)
#define PUSH_POP_SYMBOL_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH + PUSH_POP_INDEX_MAX_DIGIT + 1)

void writeArithmethicUnaryOperation(FILE* fpAsm, char *comp);
void writeArithmethicBinaryOperation(FILE* fpAsm, char *comp);
void writeArithmethicCondition(FILE* fpAsm, char *skipLabel, char *jump);

void writePushSymbol(FILE* fpAsm, char *symbol, int index);
void writePopSymbol(FILE* fpAsm, char *symbol, int index);
void writePushRegister(FILE* fpAsm, int registerNumber);
void writePopRegister(FILE* fpAsm, int registerNumber);

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
    char indexStr[PUSH_POP_INDEX_MAX_DIGIT + 1];
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

// push Memory[Memory[LCL]+index]
void writePushLocal(FILE* fpAsm, int index) { writePushSymbol(fpAsm, "LCL", index); }
// pop Memory[Memory[LCL]+index]
void writePopLocal(FILE* fpAsm, int index)  { writePopSymbol(fpAsm, "LCL", index); }

// push Memory[Memory[ARG]+index]
void writePushArgument(FILE* fpAsm, int index) { writePushSymbol(fpAsm, "ARG", index); }
// pop Memory[Memory[ARG]+index]
void writePopArgument(FILE* fpAsm, int index)  { writePopSymbol(fpAsm, "ARG", index); }

// push Memory[Memory[THIS]+index]
void writePushThis(FILE* fpAsm, int index) { writePushSymbol(fpAsm, "THIS", index); }
// pop Memory[Memory[THIS]+index]
void writePopThis(FILE* fpAsm, int index)  { writePopSymbol(fpAsm, "THIS", index); }

// push Memory[Memory[THAT]+index]
void writePushThat(FILE* fpAsm, int index) { writePushSymbol(fpAsm, "THAT", index); }
// pop Memory[Memory[THAT]+index]
void writePopThat(FILE* fpAsm, int index)  { writePopSymbol(fpAsm, "THAT", index); }

// push R{3+index}
void writePushPointer(FILE* fpAsm, int index) { writePushRegister(fpAsm, 3 + index); }
// pop R{3+index}
void writePopPointer(FILE* fpAsm, int index)  { writePopRegister(fpAsm, 3 + index); }

// push R{5+index}
void writePushTemp(FILE* fpAsm, int index) { writePushRegister(fpAsm, 5 + index); }
// pop R{5+index}
void writePopTemp(FILE* fpAsm, int index)  { writePopRegister(fpAsm, 5 + index); }

// push Memory[vmFileName.index]
void writePushStatic(FILE* fpAsm, char *vmFileName, int index)
{
    char symbol[PUSH_POP_SYMBOL_MAX_LENGTH + 1];
    sprintf(symbol, "%s.%d", vmFileName, index);

    fputslist(
        fpAsm,
        // Memory[Memory[SP]] <- Memory[symbol]
        "@", symbol, "\n",
        "D=M\n",
        "@SP\n",
        "A=M\n",
        "M=D\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// pop Memory[vmFileName.index]
void writePopStatic(FILE* fpAsm, char *vmFileName, int index)
{
    char symbol[PUSH_POP_SYMBOL_MAX_LENGTH + 1];
    sprintf(symbol, "%s.%d", vmFileName, index);

    fputslist(
        fpAsm,
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // Memory[symbol] <- Memory[Memory[SP]]
        "@SP\n",
        "A=M\n",
        "D=M\n",
        "@", symbol, "\n",
        "M=D\n",
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

// push Memory[Memory[Symbol]+index]
void writePushSymbol(FILE* fpAsm, char *symbol, int index)
{
    char indexStr[PUSH_POP_INDEX_MAX_DIGIT + 1];
    sprintf(indexStr, "%d", index);

    fputslist(
        fpAsm,
        "// push symbol ", symbol, " ", indexStr, "\n",
        // R13 <- Memory[Symbol]+index
        "@", indexStr, "\n",
        "D=A\n",
        "@", symbol, "\n",
        "D=D+M\n",
        "@R13\n",
        "M=D\n",
        // Memory[Memory[SP]] <- Memory[R13] 
        "A=M\n",
        "D=M\n",
        "@SP\n",
        "A=M\n",
        "M=D\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// pop Memory[Memory[Symbol]+index]
void writePopSymbol(FILE* fpAsm, char *symbol, int index)
{
    char indexStr[PUSH_POP_INDEX_MAX_DIGIT + 1];
    sprintf(indexStr, "%d", index);

    fputslist(
        fpAsm,
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // R13 <- Memory[Symbol]+index
        "@", indexStr, "\n",
        "D=A\n",
        "@", symbol, "\n",
        "D=D+M\n",
        "@R13\n",
        "M=D\n",
        // Memory[R13] <- Memory[Memory[SP]]
        "@SP\n",
        "A=M\n",
        "D=M\n",
        "@R13\n",
        "A=M\n",
        "M=D\n",
        NULL
    );
}

// push R{registerNumber}
void writePushRegister(FILE* fpAsm, int registerNumber)
{
    char symbol[8];
    sprintf(symbol, "R%d", registerNumber);

    fputslist(
        fpAsm,
        // Memory[Memory[SP]] <- register
        "@", symbol, "\n",
        "D=M\n",
        "@SP\n",
        "A=M\n",
        "M=D\n",
        // Memory[SP] += 1
        "@SP\n",
        "M=M+1\n",
        NULL
    );
}

// pop R{registerNumber}
void writePopRegister(FILE* fpAsm, int registerNumber)
{
    char symbol[8];
    sprintf(symbol, "R%d", registerNumber);

    fputslist(
        fpAsm,
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // register <- Memory[Memory[SP]]
        "@SP\n",
        "A=M\n",
        "D=M\n",
        "@", symbol, "\n",
        "M=D\n",
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
