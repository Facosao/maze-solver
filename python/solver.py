from __future__ import annotations

import requests
import urllib3
import time

urllib3.disable_warnings()  # Certificado SSL inválido

import sys

sys.setrecursionlimit(10000)

"""
GET exemplo
>>> import requests
>>> api_url = "https://jsonplaceholder.typicode.com/todos/1"
>>> response = requests.get(api_url)
>>> response.json()
{'userId': 1, 'id': 1, 'title': 'delectus aut autem', 'completed': False}

POST exemplo
>>> import requests
>>> api_url = "https://jsonplaceholder.typicode.com/todos"
>>> todo = {"userId": 1, "title": "Buy milk", "completed": False}
>>> response = requests.post(api_url, json=todo)
>>> response.json()
{'userId': 1, 'title': 'Buy milk', 'completed': False, 'id': 201}
"""

API = "https://gtm.delary.dev"
ID = "grupo_i"
MAZE = "medium-maze"
api_calls = 0


class Timer:
    def __init__(self) -> None:
        self.tempo_total = 0
        self.tempo_aux = 0

    def iniciar(self):
        self.tempo_aux = time.perf_counter()

    def parar(self):
        self.tempo_total += time.perf_counter() - self.tempo_aux


class No:
    def __init__(self, id: int, anterior: No) -> None:
        self.id = id
        self.anterior = anterior
        self.explorado = False
        self.adjacencias: list[int] = []
        self.inicio: bool = False
        self.final: bool = False

    def __str__(self) -> str:
        return str(self.id) + ": " + str(self.adjacencias)


nos: dict[int, No] = {}
timer_api = Timer()


def gravar_no(resposta, anterior: int) -> int:
    global nos
    if resposta.status_code != 200:
        print("Erro:", resposta.status_code)
        print(resposta.text)
        raise RuntimeError()
    else:
        resposta: dict = resposta.json()

        if nos.get(resposta.get("pos_atual")) is not None:
            return

        novo_no = No(int(resposta.get("pos_atual")), anterior)
        novo_no.inicio = resposta.get("inicio")
        novo_no.final = resposta.get("final")

        for item in resposta.get("movimentos"):
            novo_no.adjacencias.append(item)

        nos.update({novo_no.id: novo_no})

        return novo_no.id


def iniciar() -> int:
    global api_calls
    global timer_api
    dados = dict(id=ID, labirinto=MAZE)

    timer_api.iniciar()
    resposta = requests.post(API + "/iniciar", json=dados, verify=False)
    timer_api.parar()
    api_calls += 1

    return gravar_no(resposta, (-1))


def movimentar(indice: int, anterior: int) -> None:
    global api_calls
    global timer_api
    dados = dict(id=ID, labirinto=MAZE, nova_posicao=indice)

    timer_api.iniciar()
    resposta = requests.post(API + "/movimentar", json=dados, verify=False)
    timer_api.parar()
    api_calls += 1

    gravar_no(resposta, anterior)


def validar_caminho(caminho: list[int]) -> None:
    global api_calls
    global timer_api

    dados = dict(id=ID, labirinto=MAZE, todos_movimentos=caminho)

    timer_api.iniciar()
    resposta = requests.post(API + "/validar_caminho", json=dados, verify=False)
    timer_api.parar()
    api_calls += 1

    if resposta.status_code != 200:
        print(resposta.status_code)
        print(resposta.text)
        raise RuntimeError()
    else:
        resposta: dict = resposta.json()
        print("--- Qtd. movimentos:", resposta.get("quantidade_movimentos"))
        print("--- Caminho valido:", resposta.get("caminho_valido"))


def DFS_status(indice: int) -> None:
    global api_calls

    print("\r                                                       ", end="")
    print("\r--- API Calls: %d | pos_atual: %d" % (api_calls, indice), end="")


def DFS(indice: int, anterior: int, inicio=False):
    global nos
    DFS_status(indice)

    if inicio is False:
        # movimentar para este nó
        movimentar(indice, anterior)

    no = nos.get(indice)
    no.explorado = True

    # explorar adjacências
    for adj in no.adjacencias:
        no_adj = nos.get(adj)

        if no_adj is None:
            DFS(adj, indice)
        else:
            if no_adj.explorado is False:
                DFS(adj, indice)

    if no.inicio is False:
        # voltar ao nó anterior
        movimentar(no.anterior, indice)

    DFS_status(indice)


def restaurar_nos() -> None:
    global nos

    for values in nos.values():
        values.explorado = False


def BFS(raiz: int) -> int:
    global nos
    no_raiz = nos.get(raiz)
    no_raiz.explorado = True

    fila: list[int] = []
    fila.append(no_raiz.id)

    while len(fila) > 0:
        no = nos.get(fila.pop(0))
        if no.final is True:
            return no.id

        for adj in no.adjacencias:
            no_adj = nos.get(adj)

            if no_adj.explorado is False:
                no_adj.explorado = True
                no_adj.anterior = no.id
                fila.append(no_adj.id)

    raise RuntimeError()


def encontrar_caminho(final: int) -> list[int]:
    global nos
    caminho: list[int] = []
    aux = nos.get(final)

    while True:
        if aux.inicio is False:
            caminho.append(aux.id)
            aux = nos.get(aux.anterior)
        else:
            caminho.append(aux.id)
            break

    caminho.reverse()
    return caminho


if __name__ == "__main__":
    timer_main = Timer()
    timer_main.iniciar()

    print("maze-solver (Python)")
    print('1 - Fazendo chamada inicial (Labirinto: "', MAZE, '")', sep="")
    indice_inicial = iniciar()

    print("2 - Explorando o labirinto (API) com o DFS")
    DFS(indice_inicial, (-1), inicio=True)
    print("")  # Nova linha após chamadas recursivas

    for value in nos.values():
        print("---", value)

    print("3 - Resetando o estado dos nos")
    restaurar_nos()

    print("4 - Explorando o labirinto (RAM) com o BFS")
    indice_final = BFS(indice_inicial)

    print("5 - Encontrando o menor caminho")
    menor_caminho = encontrar_caminho(indice_final)
    print("---", menor_caminho)

    print("6 - Validando o menor caminho encontrado")
    validar_caminho(menor_caminho)

    timer_main.parar()

    print("7 - Estatisticas finais")
    print("--- API Calls:", api_calls)
    print("--- Tempo total do programa   : %.3f segundos" % timer_main.tempo_total)
    print("--- Tempo total das API Calls : %.3f segundos" % timer_api.tempo_total)
    proporcao = (timer_api.tempo_total * 100) / timer_main.tempo_total
    print("--- (%.2f%% do total do programa)" % proporcao)

"""
Não é possível usar o BFS puro para explorar
o labirinto e achar o menor caminho simultâneamente.

O algoritmo se movimenta para preencher as informações do
nó atual, sem levar em consideração as adjacências do nó anterior. 

No "sample_maze", a sequência de movimentos que o algoritmo
faria seria 8 -> 4 -> 5 -> 6, porém, o número 6 não faz parte da
lista de adjacências do nó #5.

Solução: Usar DFS para explorar os nós do labirinto, observando que
o algoritmo deve voltar para o nó anterior, ao final de cada caminho explorado.

Usar BFS (ou outro algoritmo) para navegar livremente pelos nós na memória
e achar o menor caminho possível.
"""

"""
Solução do sample_maze (BFS)

8, 4, 5, 6, 10, 9

9, 3, 7
8, 4, 5, 6, 10, 9, 3
8 -> 4, 5, 6, 10
4 -> 5, 8 ,9
5 -> 3, 4, 8
6 -> 7, 8, 10
10 -> 3, 6, 8
9 -> FIM

Menor caminho: [8, 4, 9]
"""
