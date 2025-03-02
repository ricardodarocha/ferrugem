// --- Teste
classe Cinco {}

var c = Cinco();

// Atribui uma funcao anonima para o objeto
c.fn = fun (a) { retorna a + 3; };

var cinco = c.fn(2);
saida cinco;

// --- Esperado
// 5