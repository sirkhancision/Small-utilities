#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    double cl, mhz;

    /* Implementador de ajuda no programa */
    if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
        printf("Calculadora de Clock Speed de memórias RAM\nFormato: Ciclo-CL MHz-RAM (16 3200)\n\nArgumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.\n");
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

    cl = atoi(argv[1]); mhz = atoi(argv[2]);

    printf("Clock Speed da RAM: %.1f ns\n", (cl*(1.0/((mhz*1E6)/2.0)))*1E9);

    return 0;
}