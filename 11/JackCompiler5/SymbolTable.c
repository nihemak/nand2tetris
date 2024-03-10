#include "SymbolTable.h"
#include "SymbolTablePrivate.h"
#include <string.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct symbol_table * SymbolTable;
struct symbol_table
{
    HashTableBucket *class_hash_table[HASH_TABLE_BUCKET_NUM];
    int static_count;
    int field_count;

    HashTableBucket *subroutine_hash_table[HASH_TABLE_BUCKET_NUM];
    int arg_count;
    int var_count;
};

SymbolTable SymbolTable_init()
{
    static struct symbol_table thisObject;

    HashTable_init(thisObject.class_hash_table);
    thisObject.static_count = 0;
    thisObject.field_count = 0;

    HashTable_init(thisObject.subroutine_hash_table);
    SymbolTable_startSubroutine(&thisObject);

    return &thisObject;
}

void SymbolTable_startSubroutine(SymbolTable thisObject)
{
    HashTable_deleteAll(thisObject->subroutine_hash_table);
    thisObject->arg_count = 0;
    thisObject->var_count = 0;
}

void SymbolTable_define(SymbolTable thisObject, char *name, char *type, SymbolTable_Kind kind)
{
    switch (kind)
    {
    case SYMBOL_TABLE_KIND_STATIC:
        HashTable_set(thisObject->class_hash_table, name, type, kind, thisObject->static_count++);
        break;
    case SYMBOL_TABLE_KIND_FIELD:
        HashTable_set(thisObject->class_hash_table, name, type, kind, thisObject->field_count++);
        break;
    case SYMBOL_TABLE_KIND_ARG:
        HashTable_set(thisObject->subroutine_hash_table, name, type, kind, thisObject->arg_count++);
        break;
    case SYMBOL_TABLE_KIND_VAR:
        HashTable_set(thisObject->subroutine_hash_table, name, type, kind, thisObject->var_count++);
        break;
    default:
        break;
    }
}

int SymbolTable_varCount(SymbolTable thisObject, SymbolTable_Kind kind)
{
    int count = 0;
    switch (kind)
    {
    case SYMBOL_TABLE_KIND_STATIC:
        count = thisObject->static_count;
        break;
    case SYMBOL_TABLE_KIND_FIELD:
        count = thisObject->field_count;
        break;
    case SYMBOL_TABLE_KIND_ARG:
        count = thisObject->arg_count;
        break;
    case SYMBOL_TABLE_KIND_VAR:
        count = thisObject->var_count;
        break;
    default:
        break;
    }
    return count;
}

SymbolTable_Kind SymbolTable_kindOf(SymbolTable thisObject, char *name)
{
    HashTableBucket *hash_bucket;
    if (HashTable_find(thisObject->subroutine_hash_table, name, &hash_bucket)) {
        return hash_bucket->kind;
    }
    else if (HashTable_find(thisObject->class_hash_table, name, &hash_bucket)) {
        return hash_bucket->kind;
    }
    return SYMBOL_TABLE_KIND_NONE;
}

void SymbolTable_typeOf(SymbolTable thisObject, char *name, char *type)
{
    HashTableBucket *hash_bucket;
    if (HashTable_find(thisObject->subroutine_hash_table, name, &hash_bucket)) {
        strcpy(type, hash_bucket->type);
    }
    else if (HashTable_find(thisObject->class_hash_table, name, &hash_bucket)) {
        strcpy(type, hash_bucket->type);
    }
    return;
}

int SymbolTable_indexOf(SymbolTable thisObject, char *name)
{
    HashTableBucket *hash_bucket;
    if (HashTable_find(thisObject->subroutine_hash_table, name, &hash_bucket)) {
        return hash_bucket->index;
    }
    if (HashTable_find(thisObject->class_hash_table, name, &hash_bucket)) {
        return hash_bucket->index;
    }
    return 0;
}

void SymbolTable_delete(SymbolTable thisObject)
{
    HashTable_deleteAll(thisObject->class_hash_table);
    HashTable_deleteAll(thisObject->subroutine_hash_table);
}
