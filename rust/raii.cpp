#include <array>
#include <stdint.h>
#include <iostream>
#include <iomanip>

#define LENGTH 15

int main(int argc, char* argv[]) {
	std::array<int32_t, LENGTH> numbers;
	numbers.fill(0);

    numbers[0] = 1;
    numbers[1] = 1;

    for (int i = 2; i < LENGTH; i++) {
        numbers[i] = numbers[i-1] + numbers[i-2];
    }

    for (int i = 0; i < LENGTH; i++) {
		std::cout << std::setw(3) << numbers[i];
    }
}
