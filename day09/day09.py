from collections import defaultdict

def read(name):
    with open(name, 'r') as file:
        return [[int(c) for c in l.strip()] for l in file.readlines()]

data = read('input.txt')

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
    m = defaultdict(lambda: (0, 0))
    for r in range(len(data)):
        for c in range(len(data[r])):
            h = get(r, c)
            if h == 9:
                continue
            if h > get(r-1, c):
                m[(r,c)] = (r-1, c)
            elif h > get(r+1, c):
                m[(r,c)] = (r+1, c)
            elif h > get(r, c-1):
                m[(r,c)] = (r, c-1)
            elif h > get(r, c+1):
                m[(r,c)] = (r, c+1)
            else:
                m[(r,c)] = (r, c)
    stable = False
    while not stable:
        stable = True
        for src, dst in m.items():
            if dst != m[dst]:
                m[src] = m[dst]
                stable = False

    sizes = defaultdict(lambda: 0)
    for src, dst in m.items():
        sizes[dst] += 1
    sizes = sorted(sizes.values())
    return sizes[-1]*sizes[-2]*sizes[-3]

print(f"Risk level: {risk_level(data)}")
print(f"Product of largest basin sizes: {basin_sizes(data)}")

