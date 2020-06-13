#ifndef _CODE_WRITER_H_INCLUDE_
#define _CODE_WRITER_H_INCLUDE_

#include "Parser.h"
#include <stdio.h>

#define CODE_WRITER_VM_FILENAME_MAX_LENGTH (32)

typedef struct code_writer * CodeWriter;

CodeWriter CodeWriter_init(FILE *fpAsm);
void CodeWriter_setFileName(CodeWriter thisObject, char *fileName);
void CodeWriter_writeArithmetic(CodeWriter thisObject, char *command);
void CodeWriter_writePushPop(
    CodeWriter thisObject,
    Parser_CommandType command,
    char *segment,
    int index
);
void CodeWriter_writeLabel(CodeWriter thisObject, char *label);
void CodeWriter_writeGoto(CodeWriter thisObject, char *label);
void CodeWriter_writeIf(CodeWriter thisObject, char *label);
void CodeWriter_writeReturn(CodeWriter thisObject);
void CodeWriter_writeFunction(CodeWriter thisObject, char *functionName, int numLocals);
void CodeWriter_close(CodeWriter thisObject);

#endif
