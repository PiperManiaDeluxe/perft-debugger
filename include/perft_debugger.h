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
typedef struct RunResult {
    /**
     * TODO_DOCS
     */
    bool successful;
    /**
     * TODO_DOCS
     */
    const char *error;
} RunResult;

/**
 * TODO_DOCS
 */
typedef struct PerftResult {
    /**
     * TODO_DOCS
     */
    bool successful;
    /**
     * TODO_DOCS
     */
    const char *error;
} PerftResult;

/**
 * Perft results for a single root move in a given position.
 */
typedef struct SingleMovePerftResult {
    /**
     * Move for this result in UCI format.
     */
    const char *uci_move;
    /**
     * Total nodes reached after this move.
     */
    size_t nodes;
} SingleMovePerftResult;

/**
 * Per-move perft results for a given root position.
 */
typedef struct PerMovePerftResult {
    /**
     * [`SingleMovePerftResult`]s for every move playable on the root position.
     */
    const struct SingleMovePerftResult *moves;
    /**
     * `moves` length.
     */
    size_t move_count;
    /**
     * Total number of nodes reached.
     */
    size_t total_nodes;
} PerMovePerftResult;

/**
 * TODO_DOCS
 */
typedef struct CPerftProvider {
    void *user_data;
    struct PerftResult (*perft)(
        void *user_data,
        const char *fen,
        const char *const *moves,
        size_t move_count,
        size_t depth,
        struct PerMovePerftResult *output
    );
} CPerftProvider;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct RunResult do_perft_test(struct CPerftProvider *provider);

/**
 * TODO_DOCS
 */
void free_run_result(struct RunResult result);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* PERFT_DEBUGGER_H */
