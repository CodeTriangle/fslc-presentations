#define LENGTH 15

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
        int* numbers = malloc(sizeof(int) * LENGTH);

        numbers[0] = 1;
        numbers[1] = 1;

        for (int i = 2; i < LENGTH; i++) {
                numbers[i] = numbers[i-1] + numbers[i-2];
        }

        for (int i = 0; i < LENGTH; i++) {
                printf("%5d\n", numbers[i]);
        }

        free(numbers);
}
