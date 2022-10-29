#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct
{
   long numerator, denominator;
} FRACAO;

static FRACAO simplify(FRACAO input)
{
   int i;

   for (i = 2; i <= 9; i++)
   {
      /* Checks if they're divisible by i, from 2 to 9 */
      while ((input.numerator % i == 0) &&
             (input.denominator % i == 0))
      {
         input.numerator   /= i;
         input.denominator /= i;
      }
   }

   return(input);
}

int main(int argc, char *argv[])
{
   FRACAO fraction;
   char * endptr;

   /* Implementador de ajuda no programa */
   if (argc > 1 && (strcmp(argv[1], "-h") == 0 ||
                    strcmp(argv[1], "--help") == 0))
   {
      puts("Simplificador de frações\n"
           "Formato de uso: x / y (Exemplo: 10 / 5)\n\n"
           "Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda.");

      return(0);
   }
   else if (argc < 4)
   {
      printf("Expressão incompleta ou formato incorreto.\n");
      exit(EXIT_FAILURE);
   }
   else if (argc > 4)
   {
      printf("Expressão possui argumentos de mais.\n");
      exit(EXIT_FAILURE);
   }

   fraction.numerator = strtol(argv[1], &endptr, 10);
   /* Checa se o denominador é igual a 0 */
   if ((fraction.denominator = strtol(argv[3], &endptr, 10)) == 0)
   {
      printf("Divisão por zero não permitida.\n");
      exit(EXIT_FAILURE);
   }

   fraction = simplify(fraction);
   printf("Fração simplificada: %ld / %ld\n",
          fraction.numerator, fraction.denominator);

   return(0);
}
