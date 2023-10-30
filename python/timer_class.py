import time


class Timer:
    def __init__(self) -> None:
        self.tempo_total: float = 0
        self.tempo_aux: float = 0

    def iniciar(self):
        self.tempo_aux = time.perf_counter()

    def parar(self):
        self.tempo_total += time.perf_counter() - self.tempo_aux
