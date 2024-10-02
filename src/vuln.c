#include "vuln.h"
#include <stdio.h>
#include <string.h>

void vulnerable_function(char *input) {
    char buffer[10];  // Buffer size of 10 bytes
    strcpy(buffer, input);  // Unsafe copy, no bounds checking
    printf("Buffer content: %s\n", buffer);
}

void call_vulnerable(char *input) {
    vulnerable_function(input);
}
