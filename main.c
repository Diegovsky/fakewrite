#include <fcntl.h>
#include <unistd.h>
#include <time.h>
#include <stdlib.h>

int main() {
    srand(time(NULL));
    const char filename[] = "out.log";
    chdir(getenv("HOME"));
    int fd = open(filename, O_WRONLY | O_CREAT);
    char text[] = "this is a tèst";
    write(fd, text, sizeof(text));
    int slen = rand() % 53 + 10;
    char* st = malloc(slen);
    for(int i = 0; i < slen; i++) {
        st[i] = 'A'+i;
    }
    write(fd, st, slen);
    free(st);
    close(fd);
    unlink(filename);
}
