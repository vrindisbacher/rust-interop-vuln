#include "vuln.h"
#include <stdio.h>
#include <string.h>

void buffer_overflow_me(char *input) {
    char buffer[10];  // Buffer size of 10 bytes
    strcpy(buffer, input);  // Unsafe copy, no bounds checking
    printf("Buffer content: %s\n", buffer);
}

int add_10(int x) {
	return x + 10;
}
