#include <stdio.h>
#include <regex.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
	if (argc <= 1) {
		exit(1);
	}

	regex_t re;
	regmatch_t match;

	if (regcomp(&re, "^[[:digit:]]+(\\.[[:digit:]]+)?$", REG_EXTENDED)) {
		perror("Error compiling regex");
		exit(1);
	}

	if (!regexec(&re, argv[1], 1, &match, 0)) {
		printf("'%s': String matches regex\n", argv[1]);
	} else {
		printf("'%s': String does not match regex\n", argv[1]);
	}

	regfree(&re);
}
