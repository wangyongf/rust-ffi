//
// Created by wangyongf on 2020/9/13.
//

#include "callrust.h"
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

int main(void) {
    print_hello_from_rust();

    uint32_t count = hm_chars("The tao of Rust.");
    printf("%s length: %d\n", "the", count);

    char *song = batman_song(5);
    printf("%s\n", song);
    free_song(song);

    uint32_t numbers[6] = {1, 2, 3, 4, 5, 6};
    uint32_t sum = sum_of_even(numbers, 6);
    printf("Sum of even: %d\n", sum);

    tuple_t initial = {.x = 10, .y = 20};
    tuple_t new = flip_things_around(initial);
    printf("Flip result: (%d, %d)\n", new.x, new.y);

    database_t *database = database_new();
    database_insert(database);
    uint32_t pop1 = database_query(database, "10086");
    uint32_t pop2 = database_query(database, "11110");
    database_free(database);
    printf("Database query result: %d\n", pop2 - pop1);
}
