#include <iostream>
#include <cstdlib>

using namespace std;

int main(int argc, char * argv[])
{
   // Paso 1: Crear variables de tipo entero sin signo
    unsigned int primero = atoi(argv[1]);
    unsigned int ultimo = atoi(argv[2]);
    unsigned int periodo = primero;
    unsigned int indice = 0;
  // Paso 2: Crear lista de enteros sin signo
    unsigned int bisiestos[(primero + ultimo)];
  // Paso 3: Ir agregando años bisiestos
    while (periodo <= ultimo)
    {
        if ((periodo % 4) == 0)
        {
            bisiestos[indice] = periodo;
            indice += 1;
        }
        periodo += 1;
    }
 // Paso 4: Mostrar cada elemento de la lista
    for (unsigned int bisiesto = 0; bisiesto < indice; bisiesto++)
    {
        cout << bisiestos[bisiesto] << endl;
    }
    return 0;
}
