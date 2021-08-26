#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int numerator, denominator;
} FRACAO;

FRACAO simplify(FRACAO input) {
	int i;
	for (i = 2; i <= 9; i++) {
		/* Checks if they're divisible by i, from 2 to 9 */
		while (((input.numerator % i == 0) && (input.denominator % i == 0))) {
			input.numerator /= i;
			input.denominator /= i;
		}
	}
	return input;
}

int main(int argc, char *argv[]) {
	FRACAO fraction;
	
	/* Implementador de ajuda no programa */
	if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
		printf("Simplificador de frações\n"
		"Formato de uso: x / y (Exemplo: 10 / 5)\n\n"
		"Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.\n");
		return 0;
	}
	else if (argc < 4) {
		printf("Expressão incompleta ou formato incorreto.\n");
		return 1;
	} 
	else if (argc > 4) {
		printf("Expressão possui argumentos de mais.\n");
		return 2;
	}

	fraction.numerator = atoi(argv[1]);
	/* Checa se o denominador é igual a 0 */
	if ((fraction.denominator = atoi(argv[3])) == 0) {
		printf("Divisão por zero não permitida.\n");
		return 3;
	}
	fraction = simplify(fraction);
	printf("Fração simplificada: %d / %d\n", fraction.numerator, fraction.denominator);

	return 0;
}
