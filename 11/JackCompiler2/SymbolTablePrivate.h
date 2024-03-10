#ifndef _SYMBOL_TABLE_PRIVATE_H_INCLUDE_
#define _SYMBOL_TABLE_PRIVATE_H_INCLUDE_

#include "SymbolTable.h"
#include "JackTokenizer.h"
#include <stdbool.h>

#define HASH_TABLE_BUCKET_NUM 50

typedef struct hash_table_bucket
{
    char key[JACK_TOKEN_SIZE];
    char type[JACK_TOKEN_SIZE];
    SymbolTable_Kind kind;
    int index;

    struct hash_table_bucket *next;
} HashTableBucket;

void HashTable_init(HashTableBucket *hash_table[]);
bool HashTable_find(HashTableBucket *hash_table[], char *key, HashTableBucket **pp_ret);
void HashTable_set(HashTableBucket *hash_table[], char* key, char *type, SymbolTable_Kind kind, int index);
void HashTable_deleteAll(HashTableBucket *hash_table[]);

#endif
