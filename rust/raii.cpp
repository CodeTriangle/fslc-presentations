#include <array>
#include <iostream>
#include <iomanip>

int main(int argc, char* argv[]) {
	std::array<int, 16> numbers;
	numbers.fill(0);

    numbers[0] = 1;
    numbers[1] = 1;

    for (int i = 2; i < numbers.size(); i++) {
        numbers[i] = numbers[i-1] + numbers[i-2];
    }

    for (int i = 0; i < numbers.size(); i++) {
		std::cout << numbers[i] << std::endl;
    }
}
