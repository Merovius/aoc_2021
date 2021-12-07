with open('input.txt', 'r') as file:
    data = list(map(int, file.read().strip().split(',')))
#data = [16,1,2,0,4,2,7,1,2,14]

def fuel_cost1(pos):
    return sum(map(lambda x: abs(x-pos), data))

def fuel_cost2(pos):
    return sum(map(lambda n: n*(n+1)//2, map(lambda x: abs(x-pos), data)))

fc = min(map(fuel_cost1, range(0, max(data))))
print(f"Minimum fuel cost: {fc}")
fc = min(map(fuel_cost2, range(0, max(data))))
print(f"Minimum fuel cost: {fc}")
