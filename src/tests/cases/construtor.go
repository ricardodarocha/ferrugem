// --- Teste
// Mostra que e possivel atribuir os valores para si mesmo via construtor
classe Abastecimento {

    ini(volume, quantidade) {
        _objeto.volume = volume;
        _objeto.quantidade = quantidade;
    }
}

var aba = Abastecimento(2, 3);
saida "" + aba.volume * aba.quantidade + "litros";

// --- Esperado
// 6 litros
