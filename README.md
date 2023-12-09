# maze-solver
Aplicação para encontrar a saída de um labirinto fornecido por uma API.

# Como usar
Para obter o programa, você pode compilar o código do mesmo ou usar o release disponível para Windows. O programa é usado através de argumentos de linha de comando. Exemplo de uso:

    .\solver.exe --maze large-maze --bfsparcial

- Argumentos obrigatórios:
    - --maze [nome do labirinto] : Especifica o nome do labirinto que será usado pelo programa. Verifique quais labirintos estão disponíveis no endpoint: https://gtm.delary.dev/labirintos
    - [abordagem] : Define qual abordagem será usada pelo programa para explorar o labirinto e encontrar um caminho. Opções disponíveis:
        - --dfs : Usa o DFS para explorar o labirinto até a saída e usa o caminho encontrado por este algoritmo.
        - --dbfs : Usa o BFS para explorar o labirinto até a saída e usa o caminho encontrado por este algoritmo.
        - --bfstotal: Usa o DFS para explorar todo o labirinto e depois usa o BFS nos vértices armazenados na memória para encontrar o caminho mais curto.
        - --bfsparcial: Usa o DFS para explorar o labirinto até a saída e depois usa o BFS nos vértices armazenados na memória para encontrar um caminho melhor que o caminho do DFS.

- Argumentos opcionais:
    - --url [url customizada] : Define uma URL customizada para tentar acessar a API. Útil para usar o programa com a API rodando localmente. Endereço padrão: https://gtm.delary.dev

# Compilando o código

Para compilar o código é necessário ter um compilador da linguagem Rust instalado em seu computador.

1. Faça um clone local deste repositório;
2. Dentro da pasta do repositório execute: ```cargo build --release```
3. O programa compilado estará em: ```..\maze-solver\target\release\solver.exe```
