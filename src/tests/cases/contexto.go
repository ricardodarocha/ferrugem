// Teste
// Este programa manipula o contexto de objetos dinâmicos
classe Mensagem {
    informa() {
        saida "Ola " + _objeto.nome;
    }
}

var m = Mensagem();
m.nome = "menina";

m.informa();

// Esperado
// "Ola menina"
