from __future__ import annotations

import requests
import urllib3

urllib3.disable_warnings()  # Certificado SSL inválido

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
MAZE = "sample_maze"

# data = requests.get(API + "/labirintos", verify=False)


def iniciar() -> No:
    dados = dict(id=ID, labirinto=MAZE)
    resposta = requests.post(API + "/iniciar", json=dados, verify=False)

    if resposta.status_code != 200:
        print(resposta.text)
        raise RuntimeError()
    else:
        novo_no = No(resposta.get("pos_atual"), -1)
        novo_no.fetch = True
        novo_no.inicio = True if resposta.get("inicio") == "true" else False
        novo_no.fim = True if resposta.get("fim") == "true" else False
        novo_no.explorado = True

        for item in resposta.get("movimentos"):
            novo_no.adjacencias.append(int(item))

        return novo_no


# print(iniciar())


class No:
    def __init__(self, id: int, anterior: No) -> None:
        self.fetch = False
        self.id = id
        self.anterior = anterior
        self.explorado = False
        self.adjacencias: list[No] = []
        self.inicio: bool = False
        self.fim: bool = False

nos: dict[int, No] = {}

def fetch_no(no: No) -> None:
    # Fazer chamada request nesta função se mover no labirinto
    dados = dict(
        id=ID,
        labirinto=MAZE,
        nova_posicao=no.id
    )
    resposta = requests.post(API + "/movimentar", json=dados, verify=False)

    if resposta.status_code != 200:
        print(resposta.text)
        raise RuntimeError()
    else:
        no.fetch = True
        no.inicio = True if resposta.get("inicio") == "true" else False
        no.fim = True if resposta.get("fim") == "true" else False
        
        for item in resposta.get("movimentos"):
            no.adjacencias.append(int(item))


def BFS() -> No:
    global nos
    fila: list[No] = []
    no_raiz = iniciar()
    nos.update({no_raiz.id: no_raiz})
    fila.append(no_raiz.id)

    while len(fila) > 0:
        no = nos.get(fila.pop(0))

        if no.fetch is False:
            fetch_no()

        if no.fim is True:
            return no

        for adj in no.adjacencias:
            if nos.get(adj) is None:
                nos.update({adj: No(adj, no.id)})
            else:
                no_adj = nos.get(adj)
                no_adj.explorado = True
                fila.append(no_adj.id)

no_final = BFS()
lista_final: list[int] = []

while True:
    if no_final.anterior != (-1):
        lista_final.append(no_final.id)
        no_final = nos.get(no_final.anterior)
    else:
        lista_final.append(no_final.id)
        break

lista_final.reverse()
print(lista_final)


"""
resp = iniciar()
print("--- 1 ---")
print(resp)
dados = dict(id=ID, labirinto=MAZE, nova_posicao=resp.get("pos_atual"))
resp2 = requests.post(API + "/movimentar", json=dados, verify=False)
print("--- 2 ---")
print(resp2.text)
"""

"""
Solução do sample_maze

8, 4, 5, 6, 10, 9

9, 3, 7

8 -> 4, 5, 6, 10
4 -> 5, 8 ,9
5 -> 3, 4, 8
6 -> 7, 8, 10
10 -> 3, 6, 8
9 -> FIM
"""
