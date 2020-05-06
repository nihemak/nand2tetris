#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "SymbolTable.h"
#include "Parser.h"

typedef struct entry Entry;
struct entry
{
    char symbol[PARSER_SYMBOL_MAX_LENGTH + 1];
    int address;
    Entry *nextEntry;
};

struct symbol_table
{
    Entry *table;
};

SymbolTable SymbolTable_init(void)
{
    static struct symbol_table thisObject;

    thisObject.table = NULL;

    return &thisObject;
}

void SymbolTable_addEntry(SymbolTable thisObject, char *symbol, int address)
{
    Entry *entry = (Entry *)malloc(sizeof(Entry));

    strcpy(entry->symbol, symbol);
    entry->address = address;
    entry->nextEntry = thisObject->table;

    thisObject->table = entry;
}

bool SymbolTable_contains(SymbolTable thisObject, char *symbol)
{
    Entry *entry = thisObject->table;
    while (entry != NULL) {
        if (strcmp(entry->symbol, symbol) == 0) {
            return true;
        }
        entry = entry->nextEntry;
    }
    return false;
}

int SymbolTable_getAddress(SymbolTable thisObject, char *symbol)
{
    Entry *entry = thisObject->table;
    while (entry != NULL) {
        if (strcmp(entry->symbol, symbol) == 0) {
            return entry->address;
        }
        entry = entry->nextEntry;
    }
    return -1;
}

void SymbolTable_delete(SymbolTable thisObject)
{
    Entry *entry = thisObject->table;
    while (entry != NULL) {
        Entry *nextEntry = entry->nextEntry;
        free(entry);
        entry = nextEntry;
    }
}
