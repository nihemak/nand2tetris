#include "CompilationEngine.h"
#include <string.h>
#include <stdbool.h>
#include <dirent.h>
#include <errno.h>

#define JACK_FILENAME_MAX_LENGTH 255
#define VM_FILENAME_MAX_LENGTH (JACK_FILENAME_MAX_LENGTH - 2)  // length('.jack') - length('.vm') = 2
#define JACK_DIRNAME_MAX_LENGTH 255

int compileByJackDir(DIR *dpJack, char *jackDirName);
int compileByJackFile(char *jackFileName);
int compile(char *vmFilePath, char *jackFilePath);
bool isJackFileName(char *jackFileName);
void createJackFilePath(char *jackDirName, char *jackFileName, char *jackFilePath);
void createVmFilePathFromDirName(char *jackDirName, char *jackFileName, char *vmFilePath);
void createVmFilePathFromJackFileName(char *jackFileName, char *vmFilePath);

int main(int argc, char *argv[]) 
{
    char *jackFileOrDirName;
    DIR *dpJack;

    if (argc != 2) {
        fprintf(stderr, "Usage: JackCompiler source\n");
        return 1;
    }

    jackFileOrDirName = argv[1];
    if (strstr(jackFileOrDirName, "/") != NULL) {
        fprintf(stderr, "Error: Jack dirname or filename is invalid. '/' can't be included. (%s)\n", jackFileOrDirName);
        return 1;
    }

    dpJack = opendir(jackFileOrDirName);
    if (dpJack != NULL) {
        int exitNo = compileByJackDir(dpJack, jackFileOrDirName);
        closedir(dpJack);
        return exitNo;
    } else if (errno == ENOTDIR) {
        return compileByJackFile(jackFileOrDirName);
    } else {
        fprintf(stderr, "Error: Jack dirname or filename is not found. (%s)\n", jackFileOrDirName);
        return 1;
    }

    return 0;
}

int compileByJackDir(DIR *dpJack, char *jackDirName)
{
    struct dirent *dEntry;
    char vmFilePath[JACK_DIRNAME_MAX_LENGTH + VM_FILENAME_MAX_LENGTH + 1];
    char jackFilePath[JACK_DIRNAME_MAX_LENGTH + JACK_FILENAME_MAX_LENGTH + 1];

    if (strlen(jackDirName) > JACK_DIRNAME_MAX_LENGTH) {
        fprintf(
            stderr, 
            "Error: Jack dirname max size is invalid. Max size is %d. (%s) is %lu\n", 
            JACK_DIRNAME_MAX_LENGTH, 
            jackDirName,
            strlen(jackDirName)
        );
        return 1;
    }

    while ((dEntry = readdir(dpJack)) != NULL) {
        char *jackFileName = dEntry->d_name;
        if (dEntry->d_type != DT_REG) {  // not file
            continue;
        }

        if (! isJackFileName(jackFileName)) {
            continue;
        }
        if (strlen(jackFileName) > JACK_FILENAME_MAX_LENGTH) {
            fprintf(
                stderr, 
                "Skip: Jack filename max size is invalid. Max size is %d. (%s) is %lu\n", 
                JACK_FILENAME_MAX_LENGTH, 
                jackFileName,
                strlen(jackFileName)
            );
            continue;
        }

        createJackFilePath(jackDirName, jackFileName, jackFilePath);
        createVmFilePathFromDirName(jackDirName, jackFileName, vmFilePath);
        if (compile(vmFilePath, jackFilePath) != 0) {
            return 0;
        }
    }

    return 0;
}

int compileByJackFile(char *jackFileName)
{
    char vmFilePath[VM_FILENAME_MAX_LENGTH];

    if (! isJackFileName(jackFileName)) {
        fprintf(stderr, "Error: Jack filename extension(.jack) is invalid. (%s)\n", jackFileName);
        return 1;
    }

    if (strlen(jackFileName) > JACK_FILENAME_MAX_LENGTH) {
        fprintf(
            stderr, 
            "Error: Jack filename max size is invalid. Max size is %d. (%s) is %lu\n", 
            JACK_FILENAME_MAX_LENGTH, 
            jackFileName,
            strlen(jackFileName)
        );
        return 1;
    }

    createVmFilePathFromJackFileName(jackFileName, vmFilePath);
    return compile(vmFilePath, jackFileName);
}

int compile(char *vmFilePath, char *jackFilePath)
{
    FILE *fpJack, *fpVm;
    CompilationEngine compilationEngine;

    if ((fpJack = fopen(jackFilePath, "r")) == NULL) {
        fprintf(stderr, "Error: jack file not found (%s)\n", jackFilePath);
        return 1;
    }

    if ((fpVm = fopen(vmFilePath, "w")) == NULL) {
        fprintf(stderr, "Error: vm file not open (%s)\n", vmFilePath);
        fclose(fpJack);
        return 1;
    }

    compilationEngine = CompilationEngine_init(fpJack, fpVm);
    CompilationEngine_compileClass(compilationEngine);

    fclose(fpVm);
    fclose(fpJack);

    return 0;
}

bool isJackFileName(char *jackFileName)
{
    size_t jackFileNameLength  = strlen(jackFileName);
    char *jackExtention = ".jack";
    size_t jackExtentionLength = strlen(jackExtention);

    if (jackFileNameLength <= jackExtentionLength) {
        return false;
    }

    // jack filename is Xxx.jack
    if (strcmp(jackFileName + jackFileNameLength - jackExtentionLength, jackExtention) != 0) {
        return false;
    }

    return true;
}

void createJackFilePath(char *jackDirName, char *jackFileName, char *jackFilePath)
{
    // jackFilePath is {jackDirName}/{jackFileName}
    strcpy(jackFilePath, jackDirName);
    strcat(jackFilePath, "/");
    strcat(jackFilePath, jackFileName);
}

void createVmFilePathFromDirName(char *jackDirName, char *jackFileName, char *vmFilePath)
{
    // vmFilePath is {jackDirName}/{jackFileName} - ".jack" + ".vm"
    strcpy(vmFilePath, jackDirName);
    strcat(vmFilePath, "/");
    strncat(vmFilePath, jackFileName, strlen(jackFileName) - strlen(".jack"));
    strcat(vmFilePath, ".vm");
}

void createVmFilePathFromJackFileName(char *jackFileName, char *vmFilePath)
{
    // VmFilePath is {jackFileName} - ".jack" + ".vm"
    size_t vmFileNamePrefixLength = strlen(jackFileName) - strlen(".jack");
    
    strncpy(vmFilePath, jackFileName, vmFileNamePrefixLength);
    vmFilePath[vmFileNamePrefixLength] = '\0';
    strcat(vmFilePath, ".vm");
}
