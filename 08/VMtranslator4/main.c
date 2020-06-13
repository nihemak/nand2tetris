#include <stdio.h>
#include <string.h>
#include <dirent.h>
#include <errno.h>
#include "Parser.h"
#include "CodeWriter.h"

// length(".asm") - length(".vm") = 1
#define ASM_FILENAME_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH + 1)
// asmfilename = vmdirname + '.asm'
#define VM_DIRNAME_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH - 4)
// asmfilename = vmfilename - '.vm' + '.asm'
#define VM_FILENAME_MAX_LENGTH (CODE_WRITER_VM_FILENAME_MAX_LENGTH - 1)

int translateByVmDir(DIR *dpVm, char *vmDirName);
int translateByVmFile(char *vmFileName);
bool isVmFileName(char *vmFileName);
void createVmFilePath(char *vmDirName, char *vmFileName, char *vmFilePath);
void createAsmFilePathFromDirName(char *vmDirName, char *asmFilePath);
void createAsmFilePathFromVmFileName(char *vmFileName, char *asmFilePath);
void translate(Parser parser, CodeWriter codeWriter);

int main(int argc, char *argv[]) 
{
    char *vmFileOrDirName;
    DIR *dpVm;

    if (argc != 2) {
        fprintf(stderr, "Usage: VMtranslator source\n");
        return 1;
    }

    vmFileOrDirName = argv[1];
    if (strstr(vmFileOrDirName, "/") != NULL) {
        fprintf(stderr, "Error: Vm dirname or filename is invalid. '/' can't be included. (%s)\n", vmFileOrDirName);
        return 1;
    }

    dpVm = opendir(vmFileOrDirName);
    if (dpVm != NULL) {
        int exitNo = translateByVmDir(dpVm, vmFileOrDirName);
        closedir(dpVm);
        return exitNo;
    } else if (errno == ENOTDIR) {
        return translateByVmFile(vmFileOrDirName);
    } else {
        fprintf(stderr, "Error: Vm dirname or filename is not found. (%s)\n", vmFileOrDirName);
        return 1;
    }

    return 0;
}

int translateByVmDir(DIR *dpVm, char *vmDirName)
{
    char asmFilePath[VM_DIRNAME_MAX_LENGTH + ASM_FILENAME_MAX_LENGTH + 1];
    char vmFilePath[VM_DIRNAME_MAX_LENGTH + VM_FILENAME_MAX_LENGTH + 1];
    int vmFileNum = 0;
    FILE *fpVm, *fpAsm;
    struct dirent *dEntry;
    Parser parser;
    CodeWriter codeWriter;

    if (strlen(vmDirName) > VM_DIRNAME_MAX_LENGTH) {
        fprintf(
            stderr, 
            "Error: Vm dirname max size is invalid. Max size is %d. (%s) is %lu\n", 
            VM_DIRNAME_MAX_LENGTH, 
            vmDirName,
            strlen(vmDirName)
        );
        return 1;
    }

    createAsmFilePathFromDirName(vmDirName, asmFilePath);
    if ((fpAsm = fopen(asmFilePath, "w")) == NULL) {
        fprintf(stderr, "Error: asm file not open (%s)\n", asmFilePath);
        return 1;
    }
    codeWriter = CodeWriter_init(fpAsm);

    while ((dEntry = readdir(dpVm)) != NULL) {
        char *vmFileName = dEntry->d_name;
        if (dEntry->d_type != DT_REG) {  // not file
            continue;
        }
        if (! isVmFileName(vmFileName)) {
            continue;
        }
        if (strlen(vmFileName) > VM_FILENAME_MAX_LENGTH) {
            fprintf(
                stderr, 
                "Skip: Vm filename max size is invalid. Max size is %d. (%s) is %lu\n", 
                VM_FILENAME_MAX_LENGTH, 
                vmFileName,
                strlen(vmFileName)
            );
            continue;
        }
        vmFileNum++;

        createVmFilePath(vmDirName, vmFileName, vmFilePath);
        if ((fpVm = fopen(vmFilePath, "r")) == NULL) {
            fprintf(stderr, "Error: vm file not found (%s)\n", vmFilePath);
            CodeWriter_close(codeWriter);
            return 1;
        }
        CodeWriter_setFileName(codeWriter, vmFileName);

        parser = Parser_init(fpVm);
        translate(parser, codeWriter);

        fclose(fpVm);

        break;
    }
    CodeWriter_close(codeWriter);

    if (vmFileNum == 0) {
        fprintf(stderr, "Error: vm file not found\n");
        return 1;
    }

    return 0;
}

int translateByVmFile(char *vmFileName)
{
    char asmFilePath[ASM_FILENAME_MAX_LENGTH];
    FILE *fpVm, *fpAsm;
    Parser parser;
    CodeWriter codeWriter;

    if (! isVmFileName(vmFileName)) {
        fprintf(stderr, "Error: Vm filename extension(.vm) is invalid. (%s)\n", vmFileName);
        return 1;
    }

    if (strlen(vmFileName) > VM_FILENAME_MAX_LENGTH) {
        fprintf(
            stderr, 
            "Error: Vm filename max size is invalid. Max size is %d. (%s) is %lu\n", 
            VM_FILENAME_MAX_LENGTH, 
            vmFileName,
            strlen(vmFileName)
        );
        return 1;
    }

    if ((fpVm = fopen(vmFileName, "r")) == NULL) {
        fprintf(stderr, "Error: vm file not found (%s)\n", vmFileName);
        return 1;
    }
    parser = Parser_init(fpVm);

    createAsmFilePathFromVmFileName(vmFileName, asmFilePath);
    if ((fpAsm = fopen(asmFilePath, "w")) == NULL) {
        fprintf(stderr, "Error: asm file not open (%s)\n", asmFilePath);
        fclose(fpVm);
        return 1;
    }
    codeWriter = CodeWriter_init(fpAsm);

    CodeWriter_setFileName(codeWriter, vmFileName);
    translate(parser, codeWriter);

    CodeWriter_close(codeWriter);
    fclose(fpVm);

    return 0;
}

bool isVmFileName(char *vmFileName)
{
    size_t vmFileNameLength  = strlen(vmFileName);
    size_t vmExtentionLength = strlen(".vm");

    if (strlen(vmFileName) <= vmExtentionLength) {
        return false;
    }

    // vm filename is Xxx.vm
    if (! (vmFileName[vmFileNameLength - 3] == '.' && 
           vmFileName[vmFileNameLength - 2] == 'v' &&
           vmFileName[vmFileNameLength - 1] == 'm')) {
        return false;
    }

    return true;
}

void createVmFilePath(char *vmDirName, char *vmFileName, char *vmFilePath)
{
    // vmFilePath is {vmDirName}/{vmFileName}
    strcpy(vmFilePath, vmDirName);
    strcat(vmFilePath, "/");
    strcat(vmFilePath, vmFileName);
}

void createAsmFilePathFromDirName(char *vmDirName, char *asmFilePath)
{
    // AsmFilePath is {vmDirName}/{vmDirName}.asm
    strcpy(asmFilePath, vmDirName);
    strcat(asmFilePath, "/");
    strcat(asmFilePath, vmDirName);
    strcat(asmFilePath, ".asm");
}

void createAsmFilePathFromVmFileName(char *vmFileName, char *asmFilePath)
{
    // AsmFilePath is {vmFileName} - ".vm" + ".asm"
    size_t asmFileNamePrefixLength = strlen(vmFileName) - strlen(".vm");
    
    strncpy(asmFilePath, vmFileName, asmFileNamePrefixLength);
    asmFilePath[asmFileNamePrefixLength] = '\0';
    strcat(asmFilePath, ".asm");
}

void translate(Parser parser, CodeWriter codeWriter)
{
    char command[PARSER_COMMAND_MAX_LENGTH + 1];
    char segment[PARSER_ARG1_MAX_LENGTH + 1];

    while (Parser_hasMoreCommands(parser)) {
        Parser_advance(parser);
        switch (Parser_commandType(parser)) {
        case PARSER_COMMAND_TYPE_C_ARITHMETIC:
            Parser_arg1(parser, command);
            CodeWriter_writeArithmetic(codeWriter, command);
            break;
        case PARSER_COMMAND_TYPE_C_PUSH:
        case PARSER_COMMAND_TYPE_C_POP:
            Parser_arg1(parser, segment);
            CodeWriter_writePushPop(codeWriter, Parser_commandType(parser), segment, Parser_arg2(parser));
            break;
        case PARSER_COMMAND_TYPE_C_LABEL:
            Parser_arg1(parser, segment);
            CodeWriter_writeLabel(codeWriter, segment);
            break;
        case PARSER_COMMAND_TYPE_C_GOTO:
            Parser_arg1(parser, segment);
            CodeWriter_writeGoto(codeWriter, segment);
            break;
        case PARSER_COMMAND_TYPE_C_IF:
            Parser_arg1(parser, segment);
            CodeWriter_writeIf(codeWriter, segment);
            break;
        case PARSER_COMMAND_TYPE_C_RETURN:
            CodeWriter_writeReturn(codeWriter);
            break;
        case PARSER_COMMAND_TYPE_C_FUNCTION:
            Parser_arg1(parser, segment);
            CodeWriter_writeFunction(codeWriter, segment, Parser_arg2(parser));
            break;
        default:
            break;
        }
    }
}
