#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned long mdc(unsigned long x, unsigned long y)
{
   unsigned long tmp;

   if (y > x)
   {
      /* Ordena de forma que x seja maior que y,
       * caso o valor de y seja maior que x */
      tmp = x;
      x   = y;
      y   = tmp;
   }

   /* Euclidean algorithm */
   while (x % y != 0)
   {
      /* Troca os valores de x e y, e realiza
       * a operação x mod y */
      tmp = x;
      x   = y;
      y   = (tmp % y);
   }

   return(y);
}

int main(int argc, char *argv[])
{
   unsigned long x, y, result;
   char *        endptr;

   /* Implementador de ajuda no programa */
   if (argc > 1 && (strcmp(argv[1], "-h") == 0 ||
                    strcmp(argv[1], "--help") == 0))
   {
      puts("Calculadora de MDC (Máximo Divisor Comum)\n"
           "Formato: x y (Exemplo: 10 5)\n\n"
           "Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.");

      return(0);
   }
   else if (argc < 3)
   {
      printf("Expressão incompleta ou formato incorreto.\n");
      exit(EXIT_FAILURE);
   }
   else if (argc > 3)
   {
      printf("Expressão possui argumentos de mais.\n");
      exit(EXIT_FAILURE);
   }

   x = strtoul(argv[1], &endptr, 10);
   y = strtoul(argv[2], &endptr, 10);
   printf("MDC: %lu\n", result = mdc(x, y));

   return(0);
}
