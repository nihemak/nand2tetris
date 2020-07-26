#include "JackTokenizer.h"
#include <string.h>
#include <dirent.h>
#include <errno.h>

#define JACK_FILENAME_MAX_LENGTH 255
#define XML_FILENAME_MAX_LENGTH JACK_FILENAME_MAX_LENGTH  // length('.jack') - length('T.xml') = 0
#define JACK_DIRNAME_MAX_LENGTH 255

int analyzeByJackDir(DIR *dpJack, char *jackDirName);
int analyzeByJackFile(char *jackFileName);
int analyze(char *xmlFilePath, char *jackFilePath);
bool isJackFileName(char *jackFileName);
void createJackFilePath(char *jackDirName, char *jackFileName, char *jackFilePath);
void createXmlFilePathFromDirName(char *jackDirName, char *jackFileName, char *xmlFilePath);
void createXmlFilePathFromJackFileName(char *jackFileName, char *xmlFilePath);
void writeTokens(FILE *fp, JackTokenizer tokenizer);
void writeToken(FILE *fp, JackTokenizer tokenizer);
void writeKeyword(FILE *fp, JackTokenizer tokenizer);
void writeSymbol(FILE *fp, JackTokenizer tokenizer);
void writeIdentifier(FILE *fp, JackTokenizer tokenizer);
void writeIntegerConstant(FILE *fp, JackTokenizer tokenizer);
void writeStringConstant(FILE *fp, JackTokenizer tokenizer);

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
    JackTokenizer tokenizer;

    if ((fpJack = fopen(jackFilePath, "r")) == NULL) {
        fprintf(stderr, "Error: jack file not found (%s)\n", jackFilePath);
        return 1;
    }

    if ((fpXml = fopen(xmlFilePath, "w")) == NULL) {
        fprintf(stderr, "Error: xml file not open (%s)\n", xmlFilePath);
        fclose(fpJack);
        return 1;
    }

    tokenizer = JackTokenizer_init(fpJack);
    writeTokens(fpXml, tokenizer);

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
    // xmlFilePath is {jackDirName}/{jackFileName} - ".jack" + "T.xml"
    strcpy(xmlFilePath, jackDirName);
    strcat(xmlFilePath, "/");
    strncat(xmlFilePath, jackFileName, strlen(jackFileName) - strlen(".jack"));
    strcat(xmlFilePath, "T.xml");
}

void createXmlFilePathFromJackFileName(char *jackFileName, char *xmlFilePath)
{
    // XmlFilePath is {jackFileName} - ".jack" + "T.xml"
    size_t xmlFileNamePrefixLength = strlen(jackFileName) - strlen(".jack");
    
    strncpy(xmlFilePath, jackFileName, xmlFileNamePrefixLength);
    xmlFilePath[xmlFileNamePrefixLength] = '\0';
    strcat(xmlFilePath, "T.xml");
}

void writeTokens(FILE *fp, JackTokenizer tokenizer)
{
    fprintf(fp, "<tokens>\n");
    while (JackTokenizer_hasMoreTokens(tokenizer)) {
        JackTokenizer_advance(tokenizer);
        writeToken(fp, tokenizer);
    }
    fprintf(fp, "</tokens>\n");
}

void writeToken(FILE *fp, JackTokenizer tokenizer)
{
    switch (JackTokenizer_tokenType(tokenizer))
    {
    case JACK_TOKENIZER_TOKEN_TYPE_KEYWORD:
        writeKeyword(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_SYMBOL:
        writeSymbol(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_IDENTIFIER:
        writeIdentifier(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_INT_CONST:
        writeIntegerConstant(fp, tokenizer);
        break;
    case JACK_TOKENIZER_TOKEN_TYPE_STRING_CONST:
        writeStringConstant(fp, tokenizer);
        break; 
    default:
        break;
    }
}

void writeKeyword(FILE *fp, JackTokenizer tokenizer)
{
    struct keyword {
        JackTokenizer_Keyword id;
        char *string;
    };
    struct keyword keywords[] = {
        { JACK_TOKENIZER_KEYWORD_CLASS,        "class" },
        { JACK_TOKENIZER_KEYWORD_METHOD,       "method" },
        { JACK_TOKENIZER_KEYWORD_FUNCTION,     "function" },
        { JACK_TOKENIZER_KEYWORD_CONSTRUCTION, "constructor" },
        { JACK_TOKENIZER_KEYWORD_INT,          "int" },
        { JACK_TOKENIZER_KEYWORD_BOOLEAN,      "boolean" },
        { JACK_TOKENIZER_KEYWORD_CHAR,         "char" },
        { JACK_TOKENIZER_KEYWORD_VOID,         "void" },
        { JACK_TOKENIZER_KEYWORD_VAR,          "var" },
        { JACK_TOKENIZER_KEYWORD_STATIC,       "static" },
        { JACK_TOKENIZER_KEYWORD_FIELD,        "field" },
        { JACK_TOKENIZER_KEYWORD_LET,          "let" },
        { JACK_TOKENIZER_KEYWORD_DO,           "do" },
        { JACK_TOKENIZER_KEYWORD_IF,           "if" },
        { JACK_TOKENIZER_KEYWORD_ELSE,         "else" },
        { JACK_TOKENIZER_KEYWORD_WHILE,        "while" },
        { JACK_TOKENIZER_KEYWORD_RETURN,       "return" },
        { JACK_TOKENIZER_KEYWORD_TRUE,         "true" },
        { JACK_TOKENIZER_KEYWORD_FALSE,        "false" },
        { JACK_TOKENIZER_KEYWORD_NULL,         "null" },
        { JACK_TOKENIZER_KEYWORD_THIS,         "this" },
    };
    JackTokenizer_Keyword id = JackTokenizer_keyword(tokenizer);

    fprintf(fp, "<keyword> ");
    for (size_t i = 0; i < sizeof(keywords) / sizeof(keywords[0]); i++) {
        if (id == keywords[i].id) {
            fprintf(fp, "%s", keywords[i].string);
            break;
        }
    }
    fprintf(fp, " </keyword>\n");
}

void writeSymbol(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_symbol(tokenizer, token);

    fprintf(fp, "<symbol> ");
    if (strcmp(token, "<") == 0) {
        fprintf(fp, "&lt;");
    } else if (strcmp(token, ">") == 0) {
        fprintf(fp, "&gt;");
    } else if (strcmp(token, "&") == 0) {
        fprintf(fp, "&amp;");
    } else {
        fprintf(fp, "%s", token);
    }
    fprintf(fp, " </symbol>\n");
}

void writeIdentifier(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_identifier(tokenizer, token);
    fprintf(fp, "<identifier> %s </identifier>\n", token);
}

void writeIntegerConstant(FILE *fp, JackTokenizer tokenizer)
{
    int intVal;
    JackTokenizer_intVal(tokenizer, &intVal);
    fprintf(fp, "<integerConstant> %d </integerConstant>\n", intVal);
}

void writeStringConstant(FILE *fp, JackTokenizer tokenizer)
{
    char token[JACK_TOKEN_SIZE];
    JackTokenizer_stringVal(tokenizer, token);
    fprintf(fp, "<stringConstant> %s </stringConstant>\n", token);
}
