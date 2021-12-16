from collections import namedtuple
from queue import PriorityQueue
import math

def read(name):
    with open(name, 'r') as file:
        return [list(map(int, l.strip())) for l in file.readlines()]

data = read('input.txt')

QEntry = namedtuple("QEntry", ["cost", "src", "dst"])

class Graph:
    def __init__(self, nodes):
        self._nodes = nodes
        self._rows = len(nodes)
        self._cols = len(nodes[0])
        self.start = (0,0)
        self.end = (self._rows-1,self._cols-1)

    def level(self, n):
        return self._nodes[n[0]][n[1]]

    def neighbors(self, n):
        if n[0] > 0:
            yield(n[0]-1, n[1])
        if n[0] < self._rows-1:
            yield(n[0]+1, n[1])
        if n[1] > 0:
            yield(n[0], n[1]-1)
        if n[1] < self._cols-1:
            yield(n[0], n[1]+1)

    def shortest_path_cost(self):
        # implements Dijkstra's Algorithm
        q = PriorityQueue()
        # we are only interested in the cost, not the actual path, so we don't
        # store where we visited a node *from*.
        visited = set()

        q.put(QEntry(cost=0, src=None, dst=self.start))
        while not q.empty():
            e = q.get()
            if e.dst in visited:
                continue
            visited.add(e.dst)
            if e.dst == self.end:
                return e.cost
            for n in self.neighbors(e.dst):
                if n in visited:
                    continue
                q.put(QEntry(cost=e.cost+self.level(n), src=e.dst, dst=n))
        raise ValueError("no path found")

def expand(data):
    out = []
    for i in range(5):
        for row in data:
            outrow = []
            for j in range(5):
                for col in row:
                    v = col+i+j
                    while v > 9:
                        v -= 9
                    outrow.append(v)
            out.append(outrow)
    return out

g = Graph(data)
print(f"Shortest path has total risk {g.shortest_path_cost()}")
data = expand(data)
g = Graph(data)
print(f"Shortest path in full cave has total risk {g.shortest_path_cost()}")
