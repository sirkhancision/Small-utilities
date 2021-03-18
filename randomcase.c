#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

int main(int argc, char *argv[]) {
	char c;
    FILE *texto;
    srand(time(0));

    if (argc > 1 && ((strcmp(argv[1], "-h")) == 0 || (strcmp(argv[1], "--help")) == 0)) {
        printf("Conversor de randomcase\nRandomiza a ordem de maiusculas e minusculas em um arquivo de texto\n\nArgumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.\n");
        return 0;
    }

    else if (argc == 1) {
        printf("Argumentos insuficientes.\n");
        return 1;
    }

    if (!(texto = fopen(argv[1], "r"))) {
        printf("Arquivo ou diretório inexistente.\n");
        return 1;
    }

    c = fgetc(texto);
    while((!feof(texto))) {
        /* if rand = 0, then do nothing */
        if ((rand() % (2 + 1 - 0) + 0) == 1 && (c >= 65 && c <= 90))
            c += 32;
        else if ((rand() % (2 + 1 - 0) + 0) == 1 && (c >= 97 && c <= 122))
            c -= 32;
        putchar(c);
        c = fgetc(texto);
    }
    fclose(texto);

    return 0;
}
