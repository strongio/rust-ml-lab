#include <stdlib.h>

void memoryLeakFunction()
{
    // Allocate memory without freeing it
    int *data = (int *)malloc(sizeof(int) * 10);

    // Do some work with the allocated memory
    for (int i = 0; i < 10; i++)
    {
        data[i] = i;
    }

    // No free(data) call to release allocated memory
}

int main()
{
    memoryLeakFunction();
    return 0;
}