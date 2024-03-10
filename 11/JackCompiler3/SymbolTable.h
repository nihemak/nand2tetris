#ifndef _SYMBOL_TABLE_H_INCLUDE_
#define _SYMBOL_TABLE_H_INCLUDE_

typedef enum {
    SYMBOL_TABLE_KIND_STATIC = 1,
    SYMBOL_TABLE_KIND_FIELD,
    SYMBOL_TABLE_KIND_ARG,
    SYMBOL_TABLE_KIND_VAR,
    SYMBOL_TABLE_KIND_NONE,
} SymbolTable_Kind;

typedef struct symbol_table * SymbolTable;

SymbolTable SymbolTable_init();
void SymbolTable_startSubroutine(SymbolTable thisObject);
void SymbolTable_define(SymbolTable thisObject, char *name, char *type, SymbolTable_Kind kind);
int SymbolTable_varCount(SymbolTable thisObject, SymbolTable_Kind kind);
SymbolTable_Kind SymbolTable_kindOf(SymbolTable thisObject, char *name);
void SymbolTable_typeOf(SymbolTable thisObject, char *name, char *type);
int SymbolTable_indexOf(SymbolTable thisObject, char *name);
void SymbolTable_delete(SymbolTable thisObject);

#endif
