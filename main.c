#include<stdio.h>

int main() {
    const char filename[] = "log.txt";
    remove(filename);
    FILE* f = fopen(filename, "w");
    fprintf(f, "%d\n", 23);
    fprintf(f, "%s\n", "Venha na minha casa e coma toda a minha família");
    fclose(f);
    remove(filename);
    return 0;
}