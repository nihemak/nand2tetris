#include "SymbolTablePrivate.h"
#include <stdlib.h>
#include <string.h>

int hash(char *s)
{
    int i = 0;
    while (*s) {
        i += *s++;
    }
    return i % HASH_TABLE_BUCKET_NUM;
}

void HashTable_init(HashTableBucket *hash_table[])
{
    for (int i = 0; i < HASH_TABLE_BUCKET_NUM; i++) {
        hash_table[i] = NULL;
    }
}

bool HashTable_find(HashTableBucket *hash_table[], char *key, HashTableBucket **pp_ret)
{
    for (HashTableBucket *p = hash_table[hash(key)]; p != NULL; p = p->next) {
        if (strcmp(key, p->key) == 0) {
            *pp_ret = p;
            return true;
        }
    }
    return false;
}

void HashTable_set(HashTableBucket *hash_table[], char* key, char *type, SymbolTable_Kind kind, int index)
{
    HashTableBucket *p;
    if (! HashTable_find(hash_table, key, &p)) {
        p = (HashTableBucket *)malloc(sizeof(HashTableBucket));
    }

    strcpy(p->key, key);
    strcpy(p->type, type);
    p->kind = kind;
    p->index = index;

    int h = hash(key);
    p->next = hash_table[h];
    hash_table[h] = p;
}

void HashTable_deleteAll(HashTableBucket *hash_table[])
{
    for (int i = 0; i < HASH_TABLE_BUCKET_NUM; i++) {
        HashTableBucket *current = hash_table[i];
        while (current != NULL) {
            HashTableBucket *next = current->next;
            free(current);
            current = next;
        }
        hash_table[i] = NULL;
    }
}
