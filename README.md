# [Fe] Ferrugem vs 0.1

Portugol sendo reescrito em Rust

![Fe](https://github.com/user-attachments/assets/56a00acf-6bd8-4e73-90f2-b7b5eca31ef1)

## Origem do nome

Ferrugem é a tradução para o português da palavra Rust, linguagem na qual este projeto foi escrito.
O símbolo [Fe] também é o símbolo do elemento Ferro.
O número do elemento [Fe] 26 também é o número de letras do alfabeto da língua portuguesa

## Inspirações

Este projeto é inspirado na primeira definição da linguagem Portugol, usada para ensinar programação e criada pelo professor Antonio Carlos Nicolodi entre 1980 e 1983

A linguagem Portugol teve forte influência de Pascal, mas mantém a sintaxe mais simples, uma característica importante para quem está aprendendo programação ou que está tendo contato com algoritmos pela primeira vez. Atualmente existem outras versões da linguagem Portugol, algumas consideradas pseudocódigo, e outras são linguagens completas, livres de contexto, com gramáticas definidas e implementações em editores ou compiladores. [Fonte: https://pt.wikipedia.org/wiki/Portugol](https://pt.wikipedia.org/wiki/Portugol)

A implementação deste projeto foi inspirado pela série do youtube
https://www.youtube.com/watch?v=Pn5RW9qFQW4&list

![preview (2)](https://github.com/user-attachments/assets/9d40f0f9-4e57-4095-a4ae-b48041203661)


## Suporte a documentação

Os códigos geram documentação no formato mermaid.js  
Ver o arquivo .doc que é gerado a cada execução do programa.

Acesse a doc [mermaid.js](https://mermaid.js.org/syntax/flowchart.html) para saber mais

*Exemplo de um fluxo de decisão gerado pelo [Fe]*

```mermaid
flowchart TD
    A[inicio] --> B{idade > 18}
    B -->|Sim| C[Maior de idade]
    C --> D[Rejuvenesce]
    D --> B
    B ---->|Não| E[fim]
```

## Contribuições

Contribuições são bem vindas, basta enviar um pull request ou enviar um e-mail para ricardodarocha@outlook.com


## Roadmap

Rust permite implementar uma série de novos recursos que a linguagem Portugol originalmente não tinha. Alguns exemplos já implementados são classes, funções anônimas, processos encadeados entre outros.

Alguns recursos ainda carecem de testes

|   |   |
|---|---|
| Olá mundo  | ✅ |
| saída  | ✅ |
| entrada  | ❌ parcialmente |
| prompt  | ✅  |
| arquivo.fe | ✅  |
| fluxo de decisão  | ✅ |
| laço  | ✅ |
| enquanto  | ✅ |
| aritimética básica  | ✅ |
| operações lógicas  | ✅ |
| geração de documentaçao  | ✅ |
| acentuação  | ❌ |
| suporte a unicode  | ❌ |
| métodos anônimos  | ✅ |
| funções  | ✅ |
| chamadas encadeadas  | ✅ |
| concatenação de strings  | ✅ |
| concatenação de strings com números  | ✅ |
| formatar a saída  | ❌ |
| consultar a data e hora do sistema  | ✅ |
| variáveis do tipo data | ❌  |
| classes | ✅  |
| propriedades | ✅  |
| métodos | ✅  |
| herança | ✅  |
| self | ✅  |
| super | ✅  |
| limpar a tela | ✅  |

> **Atenção**
> Este projeto é inteiramente experimental com fins educativos. Vários recursos ainda precisam ser testados e ajustados. Contribuições são bem vindas. Atente ao roadmap. Use o github issues para criar solicitações de ajustes e o fórum para debater as prioridades.

## Agradecimentos

Agradecimentos a
[CodeScope](https://www.youtube.com/@codescope6903)

Este projeto foi inspirado pela série do youtube
https://www.youtube.com/watch?v=Pn5RW9qFQW4&list
