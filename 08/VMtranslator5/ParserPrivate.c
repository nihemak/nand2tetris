#include "ParserPrivate.h"

bool isSpace(FILE *fpVm)
{
    int c = fgetc(fpVm);
    ungetc(c, fpVm);

    if ((char)c == ' ' || (char)c == '\t') {
        return true;
    }
    return false;
}

void skipSpaces(FILE *fpVm)
{
    while (isSpace(fpVm)) {
        fgetc(fpVm);
    }
}

bool isComment(FILE *fpVm)
{
    int c1 = fgetc(fpVm);
    if ((char)c1 != '/') {
        ungetc(c1, fpVm);
        return false;
    }
    int c2 = fgetc(fpVm);
    ungetc(c2, fpVm);
    ungetc(c1, fpVm);
    if ((char)c2 == '/') {
        return true;
    }
    return false;
}

bool isEndOfFile(FILE *fpVm)
{
    int c = fgetc(fpVm);
    ungetc(c, fpVm);

    if (c == EOF) {
        return true;
    }
    return false;
}

bool isEndOfLine(FILE *fpVm)
{
    int c = fgetc(fpVm);
    ungetc(c, fpVm);
   
    if ((char)c == '\n' || (char)c == '\r') {
        return true;
    }
    return false;
}

bool isToken(FILE *fpVm)
{
    return ! (isSpace(fpVm) || isEndOfFile(fpVm) || isEndOfLine(fpVm) || isComment(fpVm));
}

void skipEndOFLines(FILE *fpVm)
{
    while (isEndOfLine(fpVm)) {
        fgetc(fpVm);
    }
}

void skipComment(FILE *fpVm)
{
    if (isComment(fpVm)) {
        do {
            fgetc(fpVm);
        }
        while (! (isEndOfLine(fpVm) || isEndOfFile(fpVm)));
    }
}

void moveNextAdvance(FILE *fpVm)
{
    do {
        skipEndOFLines(fpVm);
        skipSpaces(fpVm);
        skipComment(fpVm);
    }
    while (isEndOfLine(fpVm));
}

void getToken(FILE *fpVm, char *token)
{
    int i = 0;
    while (isToken(fpVm)) {
        int c = fgetc(fpVm);
        token[i] = (char)c;
        i++;
    }
    token[i] = '\0';
}
