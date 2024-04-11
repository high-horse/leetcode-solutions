#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* removeKdigits(char* num, int k) {
    int len = strlen(num);
    char* result = (char*)malloc((len + 1) * sizeof(char)); // Allocate memory for result string
    int top = 0; // Stack pointer for result string
    int removed = 0; // Counter for removed digits

    for (int i = 0; i < len; i++) {
        char value = num[i];

        while (removed < k && top > 0 && result[top - 1] > value) {
            top--;
            removed++;
        }
        result[top++] = value;
    }

    // If there are still remaining digits to be removed
    while (removed < k) {
        top--;
        removed++;
    }

    // Skip leading zeros
    int leadingZeros = 0;
    while (result[leadingZeros] == '0') {
        leadingZeros++;
    }

    // Allocate memory for final result string
    char* finalResult = (char*)malloc((len + 1 - leadingZeros) * sizeof(char));

    // Copy the non-zero characters to final result
    int j = 0;
    for (int i = leadingZeros; i < top; i++) {
        finalResult[j++] = result[i];
    }
    finalResult[j] = '\0'; // Null-terminate the final result string

    free(result); // Free memory allocated for intermediate result
    
    // Check if the finalResult is empty, if so return "0"
    if (finalResult[0] == '\0') {
        free(finalResult);
        finalResult = (char*)malloc(2 * sizeof(char));
        finalResult[0] = '0';
        finalResult[1] = '\0';
    }
    return finalResult;
}

int main() {
    printf("Sss\n");
    char* number = "10200";
    int k = 1;
    char* result = removeKdigits(number, k);
    printf("%s\n", result);
    free(result); // Free memory allocated for result
    return 0;
}
