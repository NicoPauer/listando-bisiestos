#include <stdio.h>
#include <stdlib.h>

int main(int argc, char * argv[])
{
	unsigned int primero = atoi(argv[1]);
	unsigned int ultimo = atoi(argv[2]);

	unsigned int periodo = primero;

	unsigned int bisiestos[(primero + ultimo)];

	unsigned int indice = 0;

	while (periodo <= ultimo)
	{
		if ((periodo % 4) == 0)
		{
			bisiestos[indice] = periodo;
			indice += 1;
		}
		periodo += 1;
	}

	for (unsigned int bisiesto = 0; bisiesto < indice; bisiesto++)
	{
		printf("%i\n", bisiestos[bisiesto]);
	}

	return 0;
}
