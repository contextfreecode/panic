#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum pan_Error {
    pan_Error_Ok = 0,
    pan_Error_Unspecified,
    pan_Error_NotFound,
    pan_Error_Memory,
} pan_Error;

void pan_handle_abort(int sig);
void pan_main(int state);
char* pan_process_text(int id);
pan_Error pan_retrieve_text(char** result, int id);
void pan_rotate_back_char(char* text);
void pan_run(void);

int main(void) {
    pan_run();
    printf("Still alive.\n");
    // pan_main(0);
}

void pan_main(int state) {
    switch (state) {
        case 0:
            pan_run();
            // Fall through on purpose.
        case 1:
            printf("Still alive.\n");
            break;
    }
}

void pan_run(void) {
    void (*old_handler)(int) = signal(SIGABRT, pan_handle_abort);
    assert(old_handler != SIG_ERR);
    for (int id = 1; id <= 3; id += 1) {
        char* text = pan_process_text(id);
        printf("%s\n", text);
        free(text);
    }
    signal(SIGABRT, old_handler);
}

void pan_handle_abort(int sig) {
    printf("Run aborted.\n");
    pan_main(1);
}

char* pan_process_text(int id) {
    char* text;
    pan_Error error = pan_retrieve_text(&text, id);
    assert(!error);
    // if (error) abort();
    // TODO Actual Unicode code points as a future exercise.
    pan_rotate_back_char(text);
    return text;
}

const char *const pan_texts[] = {"smile", "tears"};
const size_t pan_texts_len = sizeof(pan_texts) / sizeof(*pan_texts);

pan_Error pan_retrieve_text(char** result, int id) {
    if (id <= 0 || id > pan_texts_len) return pan_Error_NotFound;
    *result = strdup(pan_texts[id - 1]);
    // assert(*result);
    if (!*result) return pan_Error_Memory;
    return pan_Error_Ok;
}

void pan_rotate_back_char(char* text) {
    size_t len = strlen(text);
    if (!len) return;
    char first = text[0];
    for (size_t index = 1; index < len; index += 1) {
        text[index - 1] = text[index];
    }
    text[len - 1] = first;
}
