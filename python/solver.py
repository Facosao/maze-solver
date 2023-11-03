from api import API
from graph import Graph
from timer_class import Timer


def solver(address: str | None, maze: str | None) -> None:
    timer_main = Timer()
    api = API(address, maze)
    graph = Graph()

    print("1 - Fazendo chamada inicial (URL: %s | Labirinto: %s)" % (api.url, api.maze))
    indice_inicial = api.iniciar(graph.vertices)

    print("2 - Explorando o labirinto (API) com o DFS")
    graph.dfs(api, indice_inicial, (-1), inicio=True)
    print("")  # Nova linha após chamadas recursivas

    for value in graph.vertices.values():
        print("---", value)

    print("3 - Resetando o estado dos nos")
    graph.restaurar_nos()

    print("4 - Explorando o labirinto (RAM) com o BFS")
    indice_final = graph.bfs(indice_inicial)

    print("5 - Encontrando o menor caminho")
    menor_caminho = graph.encontrar_caminho(indice_final)
    print("---", menor_caminho)

    print("6 - Validando o menor caminho encontrado")
    api.validar_caminho(menor_caminho)

    timer_main.parar()

    print("7 - Estatisticas finais")
    print("--- API Calls:", api.n_calls)
    print("--- Tempo total do programa   : %.3f segundos" % timer_main.tempo_total)
    print("--- Tempo total das API Calls : %.3f segundos" % api.timer.tempo_total)
    proporcao = (api.timer.tempo_total * 100) / timer_main.tempo_total
    print("--- (%.2f%% do total do programa)" % proporcao)
