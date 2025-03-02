// --- Teste
fun add1(a) {
    retorna a + 1;
}

fun add2(a) {
    retorna a + 1;
}

var b = 1 |> add1 |> fun (a) { retorna a + 2; };


saida b;


// --- Esperado
// 4