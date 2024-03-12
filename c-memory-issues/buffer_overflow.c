#include <stdio.h>
#include <string.h>

void vulnerableFunction(char *input)
{
    char buffer[10];
    unsigned short overflow = 2024;
    strcpy(buffer, input); // Unsafe
    printf("Buffer content: %s\n", buffer);
}

int main(int argc, char *argv[])
{
    vulnerableFunction(argv[1]);
    return 0;
}