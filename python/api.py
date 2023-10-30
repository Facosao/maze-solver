from requests import Response
from typing import TypedDict

import requests
import urllib3

from timer_class import Timer
from vertice import Vertice

urllib3.disable_warnings()  # Certificado SSL inválido

ID = "grupo_i"

RMovimentar = TypedDict(
    "RMovimentar",
    {"pos_atual": int, "inicio": bool, "final": bool, "movimentos": list[int]},
)

RValidarCaminho = TypedDict(
    "RValidarCaminho", {"caminho_valido": bool, "quantidade_movimentos": int}
)


class API:
    def __init__(self, address: str | None, maze_id: str | None) -> None:
        self.url: str = "https://gtm.delary.dev"
        self.maze: str = "maze-sample"
        self.n_calls: int = 0
        self.timer: Timer = Timer()

        if address is not None:
            self.url = address

        if maze_id is not None:
            self.maze = maze_id

    def gravar_no(
        self, vertices: dict[int, Vertice], resp: Response, anterior: int
    ) -> int | None:
        if resp.status_code != 200:
            print("Erro:", resp.status_code)
            print(resp.text)
            raise RuntimeError("Erro durante a gravacao do no!")
        else:
            resposta: RMovimentar = resp.json()

            if vertices.get(resposta.get("pos_atual")) is not None:
                return

            novo_vertice = Vertice(int(resposta.get("pos_atual")), anterior)
            novo_vertice.inicio = resposta.get("inicio")
            novo_vertice.final = resposta.get("final")

            for item in resposta.get("movimentos"):
                novo_vertice.adjacencias.append(item)

            vertices.update({novo_vertice.id: novo_vertice})

            return novo_vertice.id

    def iniciar(self, vertices: dict[int, Vertice]) -> int | None:
        dados = dict(id=ID, labirinto=self.maze)

        self.timer.iniciar()
        resposta = requests.post(self.url + "/iniciar", json=dados, verify=False)
        self.timer.parar()
        self.n_calls += 1

        return self.gravar_no(vertices, resposta, (-1))

    def movimentar(
        self, vertices: dict[int, Vertice], indice: int, anterior: int
    ) -> int | None:
        dados = dict(id=ID, labirinto=self.url, nova_posicao=indice)

        self.timer.iniciar()
        resposta = requests.post(self.url + "/movimentar", json=dados, verify=False)
        self.timer.parar()
        self.n_calls += 1

        return self.gravar_no(vertices, resposta, anterior)

    def validar_caminho(self, caminho: list[int]) -> None:
        dados = dict(id=ID, labirinto=self.maze, todos_movimentos=caminho)

        self.timer.iniciar()
        resposta = requests.post(
            self.url + "/validar_caminho", json=dados, verify=False
        )
        self.timer.parar()
        self.n_calls += 1

        if resposta.status_code != 200:
            print(resposta.status_code)
            print(resposta.text)
            raise RuntimeError("Erro durante a validacao do caminho!")
        else:
            resp_json: RValidarCaminho = resposta.json()
            print("--- Qtd. movimentos:", resp_json.get("quantidade_movimentos"))
            print("--- Caminho valido:", resp_json.get("caminho_valido"))
