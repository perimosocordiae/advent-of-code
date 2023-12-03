/* Usage:
clang -DPART=1 day01.c && ./a.out
clang -DPART=2 day01.c && ./a.out
rm -f a.out
*/

#include <string.h>
#include <stdio.h>

#ifndef PART
#define PART 1
#endif

char is_digit(char* str) {
  if (*str >= '0' && *str <= '9') return *str;
#if PART == 2
  if (strncmp(str, "one", 3) == 0) return '1';
  if (strncmp(str, "two", 3) == 0) return '2';
  if (strncmp(str, "three", 5) == 0) return '3';
  if (strncmp(str, "four", 4) == 0) return '4';
  if (strncmp(str, "five", 4) == 0) return '5';
  if (strncmp(str, "six", 3) == 0) return '6';
  if (strncmp(str, "seven", 5) == 0) return '7';
  if (strncmp(str, "eight", 5) == 0) return '8';
  if (strncmp(str, "nine", 4) == 0) return '9';
#endif
  return 0;
}

int main(int argc, char** argv) {
  // Open file inputs/01.full for reading.
  FILE* file = fopen("inputs/01.full", "r");
  if (file == NULL) {
    fprintf(stderr, "Failed to open file.\n");
    return 1;
  }

  // Iterate over each line in the file.
  char buffer[256] = {0};
  size_t bufsize = sizeof(buffer);
  int sum_numbers = 0;
  while (fgets(buffer, bufsize, file) != NULL) {
    // Find the first digit in the line.
    char* pos = buffer;
    char digit = is_digit(pos++);
    while (digit == 0) {
      digit = is_digit(pos++);
    }
    const int first_digit = digit - '0';

    // Find the last digit in the line.
    pos = buffer + strlen(buffer) - 1;
    digit = is_digit(pos--);
    while (digit == 0) {
      digit = is_digit(pos--);
    }
    const int last_digit = digit - '0';

    // Convert to a two-digit number and add to the sum.
    const int number = first_digit * 10 + last_digit;
    sum_numbers += number;
  }

  printf("%d\n", sum_numbers);
  return 0;
}
