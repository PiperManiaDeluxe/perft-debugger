#ifndef PERFT_DEBUGGER_H
#define PERFT_DEBUGGER_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * TODO_DOCS
 */
typedef struct FFISlice {
    const uint8_t *data;
    size_t len;
} FFISlice;

/**
 * TODO_DOCS
 */
typedef struct FFIBuffer {
    uint8_t *data;
    size_t len;
} FFIBuffer;

/**
 * TODO_DOCS
 */
typedef struct CPerftProvider {
    void *user_data;
    int (*perft)(
        void *user_data,
        const char *fen,
        const struct FFISlice *moves,
        size_t move_count,
        size_t depth,
        struct FFIBuffer *output,
        struct FFIBuffer *error
    );
    void (*free_buffer)(void *user_data, struct FFIBuffer buffer);
} CPerftProvider;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int do_perft_test(struct CPerftProvider *provider);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* PERFT_DEBUGGER_H */
