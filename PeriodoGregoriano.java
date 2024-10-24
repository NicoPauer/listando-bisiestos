public class PeriodoGrgoriano
{
  
    public PeriodoGregoriano(int inicio, int fin)
    {
      
      private int primero = inicio;
      
      private int ultimo = fin;

      private int periodo = primero;
      
      private int bisiestos[];

    }

    public int[] bisesto()
    {
      /**
        @author Nico Pauer
        Devuelve una lista de años bisiestos.
      **/
        while (periodo <= utlimo)
        {
          // Un año es bisiesto si es múltiplo de 4 (el resto entre año y 4 dá cero)
            if ((periodo % 4) == 0)
            {
              // Agrego año si es bisiesto a la lista de años bisiestos
                bisiestos.push(periodo);
            }
          // Siempre incremento en uno el periodo para que termine el bucle
            periodo += 1;
        }
      // Devuelvo lista de años bisiestos
        return bisiestos;
    }
        
}
