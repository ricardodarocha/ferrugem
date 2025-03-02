// Teste
// Este teste avalia uma condição aninhada
fun aninhada(a) {
  se (a < 3) {
    se (a > 1) {
       retorna a;
    }
  }
  senao
  {
    a = a + 2;
    retorna a;
  }

  retorna -1;
}

saida aninhada(2);
saida aninhada(1);

// --- Esperado
// 2
// 3