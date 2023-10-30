from __future__ import annotations


class Vertice:
    def __init__(self, id: int, anterior: int) -> None:
        self.id: int = id
        self.anterior: int = anterior
        self.explorado: bool = False
        self.adjacencias: list[int] = []
        self.inicio: bool = False
        self.final: bool = False

    def __str__(self) -> str:
        return str(self.id) + ": " + str(self.adjacencias)
