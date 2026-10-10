#include "perft_debugger.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int only_plays_e2e4(
    void *user_data,
    const char *fen,
    const FFISlice *moves,
    size_t move_count,
    size_t depth,
    FFIBuffer *output,
    FFIBuffer *error
) {
    (void)user_data;
    (void)moves;

    if (output == NULL || error == NULL) {
        return 2;
    }

    *output = (FFIBuffer){0};
    *error = (FFIBuffer){0};

    fprintf(stderr, "C perft callback called for FEN: %s\n", fen);
    fprintf(stderr, "Depth: %zu; moves to replay: %zu\n", depth, move_count);

    /* Return a demonstration error message for Rust to handle. */
    static const char message[] = "Demo: C perft callback was reached";

    error->data = malloc(sizeof(message) - 1);
    if (error->data == NULL) {
        return 3;
    }

    memcpy(error->data, message, sizeof(message) - 1);
    error->len = sizeof(message) - 1;

    return 1;
}

static void c_free_buffer(void *user_data, FFIBuffer buffer) {
    (void)user_data;
    free(buffer.data);
}

int main() {
    printf("Hello world!\n");

    CPerftProvider provider = {
        .user_data = NULL,
        .perft = only_plays_e2e4,
        .free_buffer = c_free_buffer,
    };

    int status = do_perft_test(&provider);

    printf("do_perft_thing returned status %d\n", status);

    return EXIT_SUCCESS;
}
