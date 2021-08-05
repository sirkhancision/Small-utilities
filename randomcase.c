#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

void rndcase(char input);

int main(int argc, char *argv[]) {
	int c;
	FILE *texto;
	srand(time(0));

	if (argc == 1) {
		printf("Argumentos insuficientes.\n");
		return 1;
	}
	else if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
		printf("Conversor de randomcase\n"
		"Randomiza a ordem de maiusculas e minusculas em um arquivo de texto\n\n"
		"Argumentos possíveis:\n\"-h\"/\"--help\" - Exibe esse texto de ajuda.\n"
		"\"-t\"/\"--stdin\" - Recebe a entrada do teclado, ao invés de por um arquivo.\n");
		return 0;
	}
	/* Input from stdin */
	else if (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--stdin") == 0) {
		while ((c = getchar()) && (c != EOF))
			rndcase(c);
		return 0;
	}

	/* Input from file */
	if (!(texto = fopen(argv[1], "r"))) {
		printf("Arquivo ou diretório inexistente.\n");
		return 1;
	}

	while ((c = fgetc(texto)) != EOF)
		rndcase(c);
	fclose(texto);

	return 0;
}

void rndcase(char input) {
	/* if rand = 0, do nothing */
	if (((rand() % (2 + 1 - 0) + 0) == 1) && islower(input))
		toupper(input);
	else if (((rand() % (2 + 1 - 0) + 0) == 1) && isupper(input))
		tolower(input);
	putchar(input);
}
