#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

void rndcase(char input) {
	int random;
	/* from 0 to 1 */
	random = rand() % 2;

	/* maybe turn lower-case into upper-case */
	if (random == 1 && islower(input))
		input = toupper(input);
	/* maybe turn upper-case into lower-case */
	else if (random == 1 && isupper(input))
		input = tolower(input);
	putchar(input);
}

int main(int argc, char *argv[]) {
	int c;
	FILE *arquivo;
	srand(time(NULL));

	if (argc == 1) {
		printf("Argumentos insuficientes.\n");
		return 1;
	}
	else if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
		printf("Conversor de randomcase\n"
		"Randomiza a ordem de maiusculas e minusculas em um arquivo de texto\n\n"
		"Argumentos possíveis:\n\"-h\"/\"--help\" - Exibe esse texto de ajuda.\n"
		"\"-t\"/\"--stdin\" - Recebe a entrada do teclado (não como parâmetro), ao invés de por um arquivo.\n");
		return 0;
	}
	/* Input from stdin */
	else if (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--stdin") == 0) {
		while ((c = getchar()) != EOF)
			rndcase(c);
		return 0;
	}

	/* Input from file */
	if (!(arquivo = fopen(argv[1], "r"))) {
		printf("Arquivo ou diretório inexistente.\n");
		return 2;
	}

	while ((c = fgetc(arquivo)) != EOF)
		rndcase(c);
	fclose(arquivo);

	return 0;
}