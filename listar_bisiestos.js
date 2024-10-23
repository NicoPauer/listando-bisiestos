// Funcion que tomando un periodo devuelve una lista de años bisiestos
function bisesto(primero, ultimo)
{
// Creo variables necesarias
    let periodo = primero;
    let bisiestos = [];
// Voy agregando bisiestos al periodo de bisiestos(bisesto)
    while (periodo <= ultimo)
    {
    // Un año es bisiesto si es multiplo de 4 (el resto entre un año y 4 dá cero)
        if ((periodo % 4) == 0)
        {
            bisiestos.push(periodo);
        }
    // Siempre incremento el periodo
        periodo += 1;
    }
// Devuelvo el bisesto
    return bisiestos;
}
