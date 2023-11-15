#include <stdio.h>
#include <stdlib.h>

int* arrayx2(int* input, int length) {
    int* output = malloc(sizeof(int) * length);

    for (int i = 0; i < length; i++) {
        output[i] = 2 * input[i];
    }

    return output;
}

int main(int argc, char* argv[]) {
    int numbers[16];

    for (int i = 0; i < 16; i++) {
        numbers[i] = i;
    }

    int* output = arrayx2(numbers, 16);

    for (int i = 0; i < 16; i++) {
        printf("%d\n", output[i]);
    }

    // this is the line you will forget.
    free(output);
}
