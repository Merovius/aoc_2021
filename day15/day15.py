from collections import namedtuple
from heapq import heappop, heappush
import math

def read(name):
    with open(name, 'r') as file:
        return [list(map(int, l.strip())) for l in file.readlines()]

data = read('input.txt')

Edge = namedtuple("Edge", ["weight", "src", "dst"])

class Graph:
    def __init__(self, nodes):
        self._nodes = nodes
        self._rows = len(nodes)
        self._cols = len(nodes[0])
        self.start = (0,0)
        self.end = (self._rows-1,self._cols-1)

    def level(self, n):
        return self._nodes[n[0]][n[1]]

    @property
    def nodes(self):
        for row in range(self._rows):
            for col in range(self._cols):
                yield (row, col)

    def neighbors(self, n):
        if n[0] > 0:
            yield(n[0]-1, n[1])
        if n[0] < self._rows-1:
            yield(n[0]+1, n[1])
        if n[1] > 0:
            yield(n[0], n[1]-1)
        if n[1] < self._cols-1:
            yield(n[0], n[1]+1)

    def edges(self, n):
        return [Edge(self.level(m), n, m) for m in self.neighbors(n)]

    def shortest_path(self):
        # implements Dijkstra's Algorithm
        heap = []
        visited = dict()
        for n in self.nodes:
            if n == self.start:
                heappush(heap, Edge(0, None, n))
            else:
                heappush(heap, Edge(math.inf, None, n))
        while len(heap) > 0:
            e = heappop(heap)
            if e.dst in visited:
                continue
            visited[e.dst] = e.src
            if e.dst == self.end:
                break
            for ne in self.edges(e.dst):
                if ne.dst in visited:
                    continue
                heappush(heap, Edge(ne.weight+e.weight, e.dst, ne.dst))
        path = []
        n = self.end
        while n is not None:
            path.append((n, self.level(n)))
            n = visited[n]
        path.reverse()
        # remove start element from path. Crude hack, but okay
        return path[1:]

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
path = g.shortest_path()
print(f"Shortest path has total risk {sum(v[1] for v in path)}")
data = expand(data)
g = Graph(data)
path = g.shortest_path()
print(f"Shortest path in full cave has total risk {sum(v[1] for v in path)}")
