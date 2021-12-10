from collections import defaultdict

def read(name):
    with open(name, 'r') as file:
        return [[int(c) for c in l.strip()] for l in file.readlines()]

data = read('input.txt')

class Point(tuple):
    def __new__(self, r, c):
        return tuple.__new__(Point, (r, c))

    def __add__(self, other):
        return Point(self[0]+other[0], self[1]+other[1])

    @property
    def val(self):
        return get(self[0], self[1])

def get(r, c):
    if r < 0 or r >= len(data):
        return 10
    if c < 0 or c >= len(data[r]):
        return 10
    return data[r][c]

def risk_level(data):
    rl = 0
    for r in range(len(data)):
        for c in range(len(data[r])):
            h = get(r, c)
            if h < get(r-1, c) and h < get(r+1,c) and h < get(r, c-1) and h < get(r, c+1):
                rl += 1+h
    return rl

def basin_sizes(data):
    labels = {}
    n = 0
    def dfs(p):
        if p in labels or p.val >= 9:
            return
        labels[p] = n
        for q in [(-1,0), (1,0), (0,-1), (0,1)]:
            dfs(p+q)

    for r in range(len(data)):
        for c in range(len(data[r])):
            p = Point(r, c)
            if p in labels or p.val >= 9:
                continue
            dfs(p)
            n += 1

    sizes = [0]*n
    for l in labels.values():
        sizes[l] += 1
    sizes.sort()
    return sizes[-1]*sizes[-2]*sizes[-3]

print(f"Risk level: {risk_level(data)}")
print(f"Product of largest basin sizes: {basin_sizes(data)}")

