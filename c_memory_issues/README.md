# Examples of C memory issues

NOTE: requires C compiler

### Buffer overflow
    
```c
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
```

Compile and run
```bash
gcc buffer_overflow.c -o buffer_overflow
```
```
./buffer_overflow "12345678901234567890"
```

### Memory Leak
```c
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
```

Compile and run
```bash
gcc memory_leak.c -o memory_leak
```
```
./memory_leak
```
