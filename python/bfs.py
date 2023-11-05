# type: strict

from __future__ import annotations


# Fire escape route - CodeChef
class Node:
    def __init__(self, id) -> None:
        self.id = id
        self.adjacent: list[Node] = []
        self.explored = False
        self.previous: Node | None = None
        self.goal = False

    def find_path(self) -> list[int]:
        path: list[int] = []
        aux = self
        while True:
            if aux.previous is not None:
                path.append(aux.id)
                aux = aux.previous
            else:
                path.append(aux.id)
                break

        path.reverse()
        return path


q: list[Node] = []
for i in range(7 + 1):
    q.append(Node(i))

q[1].adjacent = [q[2], q[3]]
q[2].adjacent = [q[1], q[4], q[5]]
q[3].adjacent = [q[1], q[6], q[7]]
q[4].adjacent = [q[2]]
q[5].adjacent = [q[2]]
q[6].adjacent = [q[3]]
q[7].adjacent = [q[3]]

q[6].goal = True


def BFS(nodes: list[Node], root: Node):
    queue: list[int] = []
    root.explored = True
    queue.append(root.id)
    start = True
    current_node: int = root.id

    while len(queue) > 0:
        # v = nodes[queue.pop(0)]
        v = nodes[queue.pop()]

        if start is False:
            if current_node == v.id:
                continue
                # Only save node info if not present
                # -- Handled by API call
            else:
                print("M", v.id)
        else:
            print("START", v.id)

        current_node = v.id

        if v.goal is True:
            return

        for node in v.adjacent:
            if node.explored is False:
                node.previous = v
                node.explored = True
                queue.extend(node.find_path())
                queue.append(node.id)
                queue.append(v.id)

        start = False


abort = False


def DFS(nodes: list[Node], v: Node, previous: Node | None, depth: int, start=False):
    global abort
    if abort is True:
        return

    v.explored = True

    if start is True:
        print("START", v.id)
    else:
        print("M", v.id)

    if v.goal is True:
        abort = True
        return

    if depth > 0:
        for adj in v.adjacent:
            if adj.explored is False:
                DFS(nodes, adj, v, depth - 1)

    if previous is not None and abort is False:
        print("V", previous.id)


def IDDFS(nodes: list[Node], v: Node):
    global abort
    i = 0

    while True:
        DFS(nodes, v, None, i, start=True)
        print("-----", i)

        if abort is True:
            break
        else:
            for node in nodes:
                node.explored = False

            i += 1


# BFS(q, q[1])
# DFS(q, q[1], None, start=True)
IDDFS(q, q[1])
