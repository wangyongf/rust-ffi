//
// Created by wangyongf on 2020/9/13.
//

#ifndef CALLRUST_CALLRUST_H
#define CALLRUST_CALLRUST_H

#include <inttypes.h>
#include <stdio.h>

void print_hello_from_rust();

uint32_t hm_chars(const char *str);

char *batman_song(uint8_t length);

void free_song(char *);

uint32_t sum_of_even(const uint32_t *numbers, size_t length);

typedef struct {
    uint32_t x;
    uint32_t y;
} tuple_t;

tuple_t flip_things_around(tuple_t);

typedef struct database_S database_t;

database_t* database_new(void);

void database_insert(database_t *);

uint32_t database_query(const database_t *, const char *);

void database_free(database_t *);

#endif //CALLRUST_CALLRUST_H
