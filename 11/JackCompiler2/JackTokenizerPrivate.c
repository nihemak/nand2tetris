#include "JackTokenizerPrivate.h"
#include <string.h>
#include <ctype.h>

bool isEndOfLine(FILE *fp);
void skipEndOFLines(FILE *fp);
bool isSpace(FILE *fp);
bool isCommentToEndOFLine(FILE *fp);
bool isCommentToClose(FILE *fp);
bool isCommentToCloseEnd(FILE *fp);
bool isString(FILE *fp);
void skipSpaces(FILE *fp);
void skipCommentToEndOFLine(FILE *fp);
void skipCommentToClose(FILE *fp);
bool isSkipToken(FILE *fp);
bool isSymbol(FILE *fp);
bool isIntConst(FILE *fp);

void moveNextToken(FILE *fp)
{
    do {
        skipEndOFLines(fp);
        skipSpaces(fp);
        skipCommentToEndOFLine(fp);
        skipCommentToClose(fp);
    }
    while (isSkipToken(fp) && ! isEndOfFile(fp)) ;
}

bool isEndOfFile(FILE *fp)
{
    int c = fgetc(fp);
    ungetc(c, fp);

    if (c == EOF) {
        return true;
    }
    return false;
}

bool isEndOfLine(FILE *fp)
{
    int c = fgetc(fp);
    ungetc(c, fp);
   
    if ((char)c == '\n' || (char)c == '\r') {
        return true;
    }
    return false;
}

void skipEndOFLines(FILE *fp)
{
    while (isEndOfLine(fp)) {
        fgetc(fp);
    }
}

bool isSpace(FILE *fp)
{
    int c = fgetc(fp);
    ungetc(c, fp);

    if ((char)c == ' ' || (char)c == '\t') {
        return true;
    }
    return false;
}

bool isCommentToEndOFLine(FILE *fp)
{
    int c1 = fgetc(fp);
    if ((char)c1 != '/') {
        ungetc(c1, fp);
        return false;
    }
    int c2 = fgetc(fp);
    ungetc(c2, fp);
    ungetc(c1, fp);
    if ((char)c2 == '/') {
        return true;
    }
    return false;
}

bool isCommentToClose(FILE *fp)
{
    int c1 = fgetc(fp);
    if ((char)c1 != '/') {
        ungetc(c1, fp);
        return false;
    }
    int c2 = fgetc(fp);
    ungetc(c2, fp);
    ungetc(c1, fp);
    if ((char)c2 == '*') {
        return true;
    }
    return false;
}

bool isCommentToCloseEnd(FILE *fp)
{
    int c1 = fgetc(fp);
    if ((char)c1 != '*') {
        ungetc(c1, fp);
        return false;
    }
    int c2 = fgetc(fp);
    ungetc(c2, fp);
    ungetc(c1, fp);
    if ((char)c2 == '/') {
        return true;
    }
    return false;
}

bool isString(FILE *fp)
{
    int c = fgetc(fp);
    ungetc(c, fp);

    if ((char)c == '"') {
        return true;
    }
    return false;
}

void skipSpaces(FILE *fp)
{
    while (isSpace(fp)) {
        fgetc(fp);
    }
}

void skipCommentToEndOFLine(FILE *fp)
{
    if (isCommentToEndOFLine(fp)) {
        do {
            fgetc(fp);
        }
        while (! (isEndOfLine(fp) || isEndOfFile(fp)));
    }
}

void skipCommentToClose(FILE *fp)
{
    if (isCommentToClose(fp)) {
        do {
            fgetc(fp);
        }
        while (! (isCommentToCloseEnd(fp)));
        fgetc(fp);
        fgetc(fp);
    }
}

bool isSkipToken(FILE *fp)
{
    return (
        isSpace(fp) ||
        isEndOfFile(fp) ||
        isEndOfLine(fp) ||
        isCommentToEndOFLine(fp) ||
        isCommentToClose(fp)
    );
}

bool isSymbol(FILE *fp)
{
    char symbols[] = {
        '{', '}', '(', ')', '[', ']', '.', ',',
        ';', '+', '-', '*', '/', '&',
        '|', '<', '>', '=', '~'
    };
    int c = fgetc(fp);
    ungetc(c, fp);

    for (size_t i = 0; i < sizeof(symbols); i++)
    {
        if ((char)c == symbols[i]) {
            return true;
        }
    }
    return false;
}

bool isIntConst(FILE *fp)
{
    int c = fgetc(fp);
    ungetc(c, fp);

    if (isdigit(c)) {
        return true;
    }
    return false;
}

bool getTokenSymbol(FILE *fp, char *token)
{
    if (! isSymbol(fp)) {
        return false;
    }

    int c = fgetc(fp);
    token[0] = (char)c;
    token[1] = '\0';

    return true;
}

bool getTokenStringConstant(FILE *fp, char *token)
{
    if (! isString(fp)) {
        return false;
    }

    int c = fgetc(fp);
    int i = 0;
    do {
        c = fgetc(fp);
        token[i] = (char)c;
        i++;
    } while (! isString(fp));
    token[i] = '\0';
    c = fgetc(fp);

    return true;
}

bool getTokenIntConstant(FILE *fp, char *token)
{
    if (! isIntConst(fp)) {
        return false;
    }

    int i = 0;
    do {
        int c = fgetc(fp);
        token[i] = (char)c;
        i++;
    } while (isIntConst(fp));
    token[i] = '\0';

    return true;
}

bool getTokenIdentifierOrKeyword(FILE *fp, char *token)
{
    int i = 0;
    while (! isSkipToken(fp) && ! isSymbol(fp)) {
        int c = fgetc(fp);
        token[i] = (char)c;
        i++;
    }
    token[i] = '\0';

    return true;
}
