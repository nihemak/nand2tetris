#include "VMWriter.h"

typedef struct vm_writer * WMWriter;
struct vm_writer
{
    FILE* fpVm;
};

void _VMWriter_writePushPop(WMWriter thisObject, char *pushPop, VMWriter_Segment segment, int index);

WMWriter WMWriter_init(FILE *fpVm)
{
    static struct vm_writer thisObject;

    thisObject.fpVm = fpVm;

    return &thisObject;
}

void VMWriter_writePush(WMWriter thisObject, VMWriter_Segment segment, int index)
{
    _VMWriter_writePushPop(thisObject, "push", segment, index);
}

void VMWriter_writePop(WMWriter thisObject, VMWriter_Segment segment, int index)
{
    _VMWriter_writePushPop(thisObject, "pop", segment, index);
}

void _VMWriter_writePushPop(WMWriter thisObject, char *op, VMWriter_Segment segment, int index)
{
    char *segmentStr;
    switch (segment)
    {
    case VM_WRITER_SEGMENT_CONST:
        segmentStr = "constant";
        break;
    case VM_WRITER_SEGMENT_ARG:
        segmentStr = "argument";
        break;
    case VM_WRITER_SEGMENT_LOCAL:
        segmentStr = "local";
        break;
    case VM_WRITER_SEGMENT_STATIC:
        segmentStr = "static";
        break;
    case VM_WRITER_SEGMENT_THIS:
        segmentStr = "this";
        break;
    case VM_WRITER_SEGMENT_THAT:
        segmentStr = "that";
        break;
    case VM_WRITER_SEGMENT_POINTER:
        segmentStr = "pointer";
        break;
    case VM_WRITER_SEGMENT_TEMP:
    default:
        segmentStr = "temp";
        break;
    }
    fprintf(thisObject->fpVm, "%s %s %d\n", op, segmentStr, index);
}

void VMWriter_writeArithmetic(WMWriter thisObject, VMWriter_Command command)
{
    char *commandStr;
    switch (command)
    {
    case VM_WRITER_COMMAND_ADD:
        commandStr = "add";
        break;
    case VM_WRITER_COMMAND_SUB:
        commandStr = "sub";
        break;
    case VM_WRITER_COMMAND_NEG:
        commandStr = "neg";
        break;
    case VM_WRITER_COMMAND_EQ:
        commandStr = "eq";
        break;
    case VM_WRITER_COMMAND_GT:
        commandStr = "gt";
        break;
    case VM_WRITER_COMMAND_LT:
        commandStr = "lt";
        break;
    case VM_WRITER_COMMAND_AND:
        commandStr = "and";
        break;
    case VM_WRITER_COMMAND_OR:
        commandStr = "or";
        break;
    case VM_WRITER_COMMAND_NOT:
    default:
        commandStr = "not";
        break;
    }
    fprintf(thisObject->fpVm, "%s\n", commandStr);
}

void VMWriter_writeLabel(WMWriter thisObject, char *label)
{
    fprintf(thisObject->fpVm, "label %s\n", label);
}

void VMWriter_writeGoto(WMWriter thisObject, char *label)
{
    fprintf(thisObject->fpVm, "goto %s\n", label);
}

void VMWriter_writeIf(WMWriter thisObject, char *label)
{
    fprintf(thisObject->fpVm, "if-goto %s\n", label);
}

void VMWriter_writeCall(WMWriter thisObject, char *name, int nArgs)
{
    fprintf(thisObject->fpVm, "call %s %d\n", name, nArgs);
}

void VMWriter_writeFunction(WMWriter thisObject, char *name, int nLocals)
{
    fprintf(thisObject->fpVm, "function %s %d\n", name, nLocals);
}

void VMWriter_writeReturn(WMWriter thisObject)
{
    fprintf(thisObject->fpVm, "return\n");
}

void VMWriter_close(WMWriter thisObject)
{
    fclose(thisObject->fpVm);
}
