use std :: env;

fn main()
{
    /* Muestra lista de años bisiestos:

        1) Crear enteros 'primero' y 'ultimo'.

        2) Asignar año de inicio en 'primero' mayor a cero.

        3) Asignar año de fin mayor a 'primero' en 'ultimo'.

        4) Crear entero 'periodo' y asignarle 'primero'.

        5) Crear lista de enteros 'bisiestos'.

        6) Mientras 'periodo' sea menor o igual a 'ultimo',
           si 'periodo' es bisiesto (múltiplo de 4) agregarlo a 'bisiestos'.
           incrementar siempre en 1 'periodo'.

        7) Mostrar cada elemento de 'bisiestos'.

    */
    // Definicion argumentos de terminal 'año de inicio'y 'año de fin'
    let argumento_terminal : Vec<String> = env :: args().collect();
    // Paso 1
    let primero:u32;
    let ultimo:u32;
    // Paso 2
    primero = argumento_terminal[1].clone().parse().expect("Error En argumento 1.");
    // Paso 3
    ultimo = argumento_terminal[2].clone().parse().expect("Error En argumento 2.");
    // Paso 4
    let mut periodo:u32 = primero;
    // Paso 5
    let mut bisiestos:[u32; 30_000] = [0; 30_000];

    let mut indice:usize = 0;
    // Paso 6
    while periodo <= ultimo
    {
        if periodo % 4 == 0
        {
            bisiestos[indice] = periodo;
            indice += 1;
        }

        periodo += 1;
    }
    // Paso 7
    for bisiesto in bisiestos
    {
        if bisiesto != 0
        {
            println!("{bisiesto}");
        }
        else
        {
            break;
        }
    }
}
