#ifndef _VM_WRITER_H_INCLUDE_
#define _VM_WRITER_H_INCLUDE_

#include <stdio.h>

typedef enum {
    VM_WRITER_SEGMENT_CONST = 1,
    VM_WRITER_SEGMENT_ARG,
    VM_WRITER_SEGMENT_LOCAL,
    VM_WRITER_SEGMENT_STATIC,
    VM_WRITER_SEGMENT_THIS,
    VM_WRITER_SEGMENT_THAT,
    VM_WRITER_SEGMENT_POINTER,
    VM_WRITER_SEGMENT_TEMP,
} VMWriter_Segment;

typedef enum {
    VM_WRITER_COMMAND_ADD = 1,
    VM_WRITER_COMMAND_SUB,
    VM_WRITER_COMMAND_NEG,
    VM_WRITER_COMMAND_EQ,
    VM_WRITER_COMMAND_GT,
    VM_WRITER_COMMAND_LT,
    VM_WRITER_COMMAND_AND,
    VM_WRITER_COMMAND_OR,
    VM_WRITER_COMMAND_NOT,
} VMWriter_Command;

typedef struct vm_writer * WMWriter;

WMWriter WMWriter_init(FILE *fpVm);
void VMWriter_writePush(WMWriter thisObject, VMWriter_Segment segment, int index);
void VMWriter_writePop(WMWriter thisObject, VMWriter_Segment segment, int index);
void VMWriter_writeArithmetic(WMWriter thisObject, VMWriter_Command command);
void VMWriter_writeLabel(WMWriter thisObject, char *label);
void VMWriter_writeGoto(WMWriter thisObject, char *label);
void VMWriter_writeIf(WMWriter thisObject, char *label);
void VMWriter_writeCall(WMWriter thisObject, char *name, int nArgs);
void VMWriter_writeFunction(WMWriter thisObject, char *name, int nLocals);
void VMWriter_writeReturn(WMWriter thisObject);
void VMWriter_close(WMWriter thisObject);

#endif
