#include <stdio.h>

// Print a table for Fahrenheit-Celsius conversion.

int main() {

    // Declaration
    float fahr, celsius;
    char endBar[] = "+--------+---------+";

    // Calc. and print table
    printf("%s\n", endBar);
    printf("| Fahr   | Celsius |\n");
    printf("%s\n", endBar);

    for (fahr = 300.0; fahr >= 0.0; fahr = fahr - 20) {
        printf("| %6.0f | %7.1f |\n", fahr, (5.0/9.0) * (fahr-32.0) );
    }

    printf("%s\n", endBar);

    return 0;
}
