#include "CompilationEngine.h"
#include <string.h>
#include <stdbool.h>
#include <dirent.h>
#include <errno.h>

#define JACK_FILENAME_MAX_LENGTH 255
#define XML_FILENAME_MAX_LENGTH (JACK_FILENAME_MAX_LENGTH - 1)  // length('.jack') - length('.xml') = 1
#define JACK_DIRNAME_MAX_LENGTH 255

int analyzeByJackDir(DIR *dpJack, char *jackDirName);
int analyzeByJackFile(char *jackFileName);
int analyze(char *xmlFilePath, char *jackFilePath);
bool isJackFileName(char *jackFileName);
void createJackFilePath(char *jackDirName, char *jackFileName, char *jackFilePath);
void createXmlFilePathFromDirName(char *jackDirName, char *jackFileName, char *xmlFilePath);
void createXmlFilePathFromJackFileName(char *jackFileName, char *xmlFilePath);

int main(int argc, char *argv[]) 
{
    char *jackFileOrDirName;
    DIR *dpJack;

    if (argc != 2) {
        fprintf(stderr, "Usage: JackAnalyzer source\n");
        return 1;
    }

    jackFileOrDirName = argv[1];
    if (strstr(jackFileOrDirName, "/") != NULL) {
        fprintf(stderr, "Error: Jack dirname or filename is invalid. '/' can't be included. (%s)\n", jackFileOrDirName);
        return 1;
    }

    dpJack = opendir(jackFileOrDirName);
    if (dpJack != NULL) {
        int exitNo = analyzeByJackDir(dpJack, jackFileOrDirName);
        closedir(dpJack);
        return exitNo;
    } else if (errno == ENOTDIR) {
        return analyzeByJackFile(jackFileOrDirName);
    } else {
        fprintf(stderr, "Error: Jack dirname or filename is not found. (%s)\n", jackFileOrDirName);
        return 1;
    }

    return 0;
}

int analyzeByJackDir(DIR *dpJack, char *jackDirName)
{
    struct dirent *dEntry;
    char xmlFilePath[JACK_DIRNAME_MAX_LENGTH + XML_FILENAME_MAX_LENGTH + 1];
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
        createXmlFilePathFromDirName(jackDirName, jackFileName, xmlFilePath);
        if (analyze(xmlFilePath, jackFilePath) != 0) {
            return 0;
        }
    }

    return 0;
}

int analyzeByJackFile(char *jackFileName)
{
    char xmlFilePath[XML_FILENAME_MAX_LENGTH];

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

    createXmlFilePathFromJackFileName(jackFileName, xmlFilePath);
    return analyze(xmlFilePath, jackFileName);
}

int analyze(char *xmlFilePath, char *jackFilePath)
{
    FILE *fpJack, *fpXml;
    CompilationEngine compilationEngine;

    if ((fpJack = fopen(jackFilePath, "r")) == NULL) {
        fprintf(stderr, "Error: jack file not found (%s)\n", jackFilePath);
        return 1;
    }

    if ((fpXml = fopen(xmlFilePath, "w")) == NULL) {
        fprintf(stderr, "Error: xml file not open (%s)\n", xmlFilePath);
        fclose(fpJack);
        return 1;
    }

    compilationEngine = CompilationEngine_init(fpJack, fpXml);
    CompilationEngine_compileClass(compilationEngine);

    fclose(fpXml);
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

void createXmlFilePathFromDirName(char *jackDirName, char *jackFileName, char *xmlFilePath)
{
    // xmlFilePath is {jackDirName}/{jackFileName} - ".jack" + ".xml"
    strcpy(xmlFilePath, jackDirName);
    strcat(xmlFilePath, "/");
    strncat(xmlFilePath, jackFileName, strlen(jackFileName) - strlen(".jack"));
    strcat(xmlFilePath, ".xml");
}

void createXmlFilePathFromJackFileName(char *jackFileName, char *xmlFilePath)
{
    // XmlFilePath is {jackFileName} - ".jack" + ".xml"
    size_t xmlFileNamePrefixLength = strlen(jackFileName) - strlen(".jack");
    
    strncpy(xmlFilePath, jackFileName, xmlFileNamePrefixLength);
    xmlFilePath[xmlFileNamePrefixLength] = '\0';
    strcat(xmlFilePath, ".xml");
}
