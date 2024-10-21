#!/usr/bin/python3

# Toma de paramtero dos años siendo el segundo mayor a primero
# Devuelve de salida una lista de años bisiestos duarnte ese periodo
import sys

# Defino parametros del comando
primero = int(sys.argv[1])

ultimo = int(sys.argv[2])
# Defino variables de control
periodo = primero

bisiestos = []

while (periodo <= ultimo):
    # Si es bisiesto lo agrego a la lis
    if ((periodo % 4) == 0):
        bisiestos.append(periodo)
    # Incremento en uno el año del perido
    periodo += 1
# Muestro los años
for bisiesto in bisiestos:
    print(bisiesto)
