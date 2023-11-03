from api import API
from vertice import Vertice


class Graph:
    def __init__(self) -> None:
        self.vertices: dict[int, Vertice] = dict()

    def dfs_status(self, api: API, indice: int) -> None:
        print("\r                                                         ", end="")
        print("\r--- API Calls: %d | pos_atual: %d" % (api.n_calls, indice), end="")

    def dfs(self, api: API, indice: int, anterior: int, inicio: bool = False):
        global nos
        self.dfs_status(api, indice)

        if inicio is False:
            # movimentar para este nó
            api.movimentar(self.vertices, indice, anterior)

        vertice = self.vertices[indice]
        vertice.explorado = True

        # explorar adjacências
        for adj in vertice.adjacencias:
            vertice_adj = self.vertices.get(adj)

            if vertice_adj is None:
                self.dfs(api, adj, indice)
            else:
                if vertice_adj.explorado is False:
                    self.dfs(api, adj, indice)

        if vertice.inicio is False:
            # voltar ao nó anterior
            api.movimentar(self.vertices, vertice.anterior, indice)

        self.dfs_status(api, indice)

    def restaurar_nos(self) -> None:
        for vertice in self.vertices.values():
            vertice.explorado = False

    def bfs(self, raiz: int) -> int:
        vertice_raiz = self.vertices[raiz]
        vertice_raiz.explorado = True

        fila: list[int] = []
        fila.append(vertice_raiz.id)

        while len(fila) > 0:
            vertice = self.vertices[fila.pop(0)]
            if vertice.final is True:
                return vertice.id

            for adj in vertice.adjacencias:
                vertice_adj = self.vertices[adj]

                if vertice_adj.explorado is False:
                    vertice_adj.explorado = True
                    vertice_adj.anterior = vertice.id
                    fila.append(vertice_adj.id)

        raise RuntimeError()

    def encontrar_caminho(self, final: int) -> list[int]:
        caminho: list[int] = []
        aux = self.vertices[final]

        while True:
            if aux.inicio is False:
                caminho.append(aux.id)
                aux = self.vertices[aux.anterior]
            else:
                caminho.append(aux.id)
                break

        caminho.reverse()
        return caminho
