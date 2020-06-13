#include "CodeWriter.h"
#include "CodeWriterPrivate.h"
#include <string.h>

#define ARITHMETIC_SKIP_LABEL_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH + 24)

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
    if (command == PARSER_COMMAND_TYPE_C_PUSH) {
        if (strcmp(segment, "constant") == 0) {
            writePushConstant(thisObject->fpAsm, index);
        }
    }
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
