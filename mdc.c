#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int mdc(int x, int y) {
	int tmp;
	if (y > x) {
		/* Ordena de forma que x seja maior que y,
		 * caso o valor de y seja maior que x */
		tmp = x;
		x = y;
		y = tmp;
	}

	/* Euclidean algorithm */
	while (x % y != 0) {
		/* Troca os valores de x e y, e realiza
		 * a operação x mod y */
		tmp = x;
		x = y;
		y = (tmp % y);
	}
	return y;
}

int main(int argc, char *argv[]) {
	int x, y, result;

	/* Implementador de ajuda no programa */
	if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
		printf("Calculadora de MDC (Máximo Divisor Comum)\n"
		"Formato: x y (Exemplo: 10 5)\n\n"
		"Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.\n");
		return 0;
	}
	else if (argc < 3) {
		printf("Expressão incompleta ou formato incorreto.\n");
		return 1;
	} 
	else if (argc > 3) {
		printf("Expressão possui argumentos de mais.\n");
		return 1;
	}

	x = atoi(argv[1]);
	y = atoi(argv[2]);
	printf("MDC: %d\n", result = mdc(x, y));

	return 0;
}
