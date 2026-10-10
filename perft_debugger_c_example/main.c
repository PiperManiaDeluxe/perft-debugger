#include "perft_debugger.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/// TODO_DOCS
typedef struct {
    SingleMovePerftResult *move_results;
} ExampleContext;

/// TODO_DOCS
static PerftResult only_plays_e2e4(
    void *user_data,
    const char *fen,
    const char *const *moves,
    size_t move_count,
    size_t depth,
    PerMovePerftResult *output
) {
    (void)user_data;
    (void)fen;
    (void)moves;
    (void)move_count;
    (void)depth;

    ExampleContext *context = user_data;

    size_t move_result_count = 1;

    context->move_results = malloc(move_result_count * sizeof(*context->move_results));
    if (context->move_results == NULL) {
        return (PerftResult){
            .successful = false,
            .error = "Could not allocate move_results",
        };
    }

    context->move_results[0] = (SingleMovePerftResult){
        .uci_move = "e2e4",
        .nodes = 1,
    };

    output->moves = context->move_results;
    output->move_count = 1;
    output->total_nodes = 1;

    return (PerftResult){.successful = true, .error = ""};
}

int main() {
    ExampleContext context = {0};

    CPerftProvider provider = {
        .user_data = &context,
        .perft = only_plays_e2e4,
    };

    RunResult result = do_perft_test(&provider);

    free(context.move_results);

    if (!result.successful) {
        if (result.error != NULL) {
            fprintf(stdout, "%s\n", result.error);
        }

        free_run_result(result);
        return EXIT_FAILURE;
    }

    free_run_result(result);
    return EXIT_SUCCESS;
}
