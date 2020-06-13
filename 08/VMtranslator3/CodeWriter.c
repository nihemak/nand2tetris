#include "CodeWriter.h"
#include "CodeWriterPrivate.h"
#include <string.h>

#define ARITHMETIC_SKIP_LABEL_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH + 24)
#define LABEL_SYMBOL_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH + 24)

void writeArithmethicEqNext(CodeWriter thisObject);
void writeArithmethicGtNext(CodeWriter thisObject);
void writeArithmethicLtNext(CodeWriter thisObject);

struct code_writer
{
    FILE* fpAsm;
    char vmFileName[CODE_WRITER_VM_FILENAME_MAX_LENGTH + 1];
    int arithmeticEqCount;
    int arithmeticGtCount;
    int arithmeticLtCount;
};

CodeWriter CodeWriter_init(FILE *fpAsm)
{
    static struct code_writer thisObject;

    thisObject.fpAsm = fpAsm;
    CodeWriter_setFileName(&thisObject, "");

    return &thisObject;
}

void CodeWriter_setFileName(CodeWriter thisObject, char *fileName)
{
    strcpy(thisObject->vmFileName, fileName);
    thisObject->arithmeticEqCount = 0;
    thisObject->arithmeticGtCount = 0;
    thisObject->arithmeticLtCount = 0;
}

void CodeWriter_writeArithmetic(CodeWriter thisObject, char *command)
{
         if (strcmp(command, "add") == 0) writeArithmethicAdd(thisObject->fpAsm);
    else if (strcmp(command, "sub") == 0) writeArithmethicSub(thisObject->fpAsm);
    else if (strcmp(command, "neg") == 0) writeArithmethicNeg(thisObject->fpAsm);
    else if (strcmp(command,  "eq") == 0) writeArithmethicEqNext(thisObject);
    else if (strcmp(command,  "gt") == 0) writeArithmethicGtNext(thisObject);
    else if (strcmp(command,  "lt") == 0) writeArithmethicLtNext(thisObject);
    else if (strcmp(command, "and") == 0) writeArithmethicAnd(thisObject->fpAsm);
    else if (strcmp(command,  "or") == 0) writeArithmethicOr(thisObject->fpAsm);
    else if (strcmp(command, "not") == 0) writeArithmethicNot(thisObject->fpAsm);
}

void CodeWriter_writePushPop(
    CodeWriter thisObject,
    Parser_CommandType command,
    char *segment,
    int index
) {
    switch (command) {
    case PARSER_COMMAND_TYPE_C_PUSH:
             if (strcmp(segment, "constant") == 0) writePushConstant(thisObject->fpAsm, index);
        else if (strcmp(segment,    "local") == 0) writePushLocal(thisObject->fpAsm, index);
        else if (strcmp(segment, "argument") == 0) writePushArgument(thisObject->fpAsm, index);
        else if (strcmp(segment,     "this") == 0) writePushThis(thisObject->fpAsm, index);
        else if (strcmp(segment,     "that") == 0) writePushThat(thisObject->fpAsm, index);
        else if (strcmp(segment,  "pointer") == 0) writePushPointer(thisObject->fpAsm, index);
        else if (strcmp(segment,     "temp") == 0) writePushTemp(thisObject->fpAsm, index);
        else if (strcmp(segment,   "static") == 0) writePushStatic(thisObject->fpAsm, thisObject->vmFileName, index);
        break;
    case PARSER_COMMAND_TYPE_C_POP:
             if (strcmp(segment,    "local") == 0) writePopLocal(thisObject->fpAsm, index);
        else if (strcmp(segment, "argument") == 0) writePopArgument(thisObject->fpAsm, index);
        else if (strcmp(segment,     "this") == 0) writePopThis(thisObject->fpAsm, index);
        else if (strcmp(segment,     "that") == 0) writePopThat(thisObject->fpAsm, index);
        else if (strcmp(segment,  "pointer") == 0) writePopPointer(thisObject->fpAsm, index);
        else if (strcmp(segment,     "temp") == 0) writePopTemp(thisObject->fpAsm, index);
        else if (strcmp(segment,   "static") == 0) writePopStatic(thisObject->fpAsm, thisObject->vmFileName, index);
        break;
    default:
        break;
    }
}

void CodeWriter_writeLabel(CodeWriter thisObject, char *label)
{
    char labelSymbol[LABEL_SYMBOL_MAX_LENGTH + 1];
    sprintf(labelSymbol, "%s$%s", thisObject->vmFileName, label);

    fputslist(
        thisObject->fpAsm,
        "(", labelSymbol, ")\n",
        NULL
    );
}

void CodeWriter_writeGoto(CodeWriter thisObject, char *label)
{
    char labelSymbol[LABEL_SYMBOL_MAX_LENGTH + 1];
    sprintf(labelSymbol, "%s$%s", thisObject->vmFileName, label);

    fputslist(
        thisObject->fpAsm,
        // goto labelSymbol
        "@", labelSymbol, "\n",
        "0;JMP\n",
        NULL
    );
}

void CodeWriter_writeIf(CodeWriter thisObject, char *label)
{
    char labelSymbol[LABEL_SYMBOL_MAX_LENGTH + 1];
    sprintf(labelSymbol, "%s$%s", thisObject->vmFileName, label);

    fputslist(
        thisObject->fpAsm,
        // Memory[SP] -= 1
        "@SP\n",
        "M=M-1\n",
        // Register <- Memory[Memory[SP]]
        "@SP\n",
        "A=M\n",
        "D=M\n",
        // if jump(Register != 0) then goto labelSymbol
        "@", labelSymbol, "\n",
        "D;JNE\n",
        NULL
    );
}

void CodeWriter_close(CodeWriter thisObject)
{
    fclose(thisObject->fpAsm);
}

void writeArithmethicEqNext(CodeWriter thisObject)
{
    char skipLabel[ARITHMETIC_SKIP_LABEL_MAX_LENGTH + 1];
    sprintf(skipLabel, "%s.SKIP_EQ.%d", thisObject->vmFileName, thisObject->arithmeticEqCount);
    writeArithmethicEq(thisObject->fpAsm, skipLabel);
    thisObject->arithmeticEqCount++;
}

void writeArithmethicGtNext(CodeWriter thisObject)
{
    char skipLabel[ARITHMETIC_SKIP_LABEL_MAX_LENGTH + 1];
    sprintf(skipLabel, "%s.SKIP_GT.%d", thisObject->vmFileName, thisObject->arithmeticGtCount);
    writeArithmethicGt(thisObject->fpAsm, skipLabel);
    thisObject->arithmeticGtCount++;
}

void writeArithmethicLtNext(CodeWriter thisObject)
{
    char skipLabel[ARITHMETIC_SKIP_LABEL_MAX_LENGTH + 1];
    sprintf(skipLabel, "%s.SKIP_LT.%d", thisObject->vmFileName, thisObject->arithmeticLtCount);
    writeArithmethicLt(thisObject->fpAsm, skipLabel);
    thisObject->arithmeticLtCount++;
}
