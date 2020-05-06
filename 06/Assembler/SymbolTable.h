#ifndef _SYMBOL_TABLE_H_INCLUDE_
#define _SYMBOL_TABLE_H_INCLUDE_

#include <stdbool.h>

typedef struct symbol_table * SymbolTable;

SymbolTable SymbolTable_init(void);
void SymbolTable_addEntry(SymbolTable thisObject, char *symbol, int address);
bool SymbolTable_contains(SymbolTable thisObject, char *symbol);
int SymbolTable_getAddress(SymbolTable thisObject, char *symbol);
void SymbolTable_delete(SymbolTable thisObject);

#endif
