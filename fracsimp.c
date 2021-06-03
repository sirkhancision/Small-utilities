#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  int numerador, denominador;
} FRACAO;

FRACAO simp(FRACAO som) {
  int i;

  for (i = 2; i <= 9; i++) {
    if ((som.numerador % i == 0) && (som.denominador % i == 0)) {
      while (((som.numerador % i == 0) && (som.denominador % i == 0))) {
        som.numerador /= i;
        som.denominador /= i;
      }
    }
  }
  return som;
}

int main(int argc, char *argv[]) {
  FRACAO frac;

  /* Implementador de ajuda no programa */
  if (argc > 1 &&
      (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
    printf("Simplificador de frações\n"
           "Formato de uso: x / y (Exemplo: 10 / 5)\n\n"
           "Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de "
           "ajuda.\n");
    return 0;
  }

  else if (argc < 4) {
    printf("Expressão incompleta ou formato incorreto.\n");
    return 1;
  } else if (argc > 4) {
    printf("Expressão possui argumentos de mais.\n");
    return 1;
  }

  frac.numerador = atoi(argv[1]);
  frac.denominador = atoi(argv[3]);
  frac = simp(frac);
  printf("Fração simplificada: %d / %d\n", frac.numerador, frac.denominador);

  return 0;
}
