file = open('input.txt', 'r')
lines = file.readlines()

#lines = ["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"]

lines = [int(l) for l in lines]

count = 0
for i in range(len(lines)-1):
    if lines[i] < lines[i+1]:
        count += 1
print("Number of increases:", count)

windows = [lines[i-1]+lines[i]+lines[i+1] for i in range(1, len(lines)-1)]
count = 0
for i in range(len(windows)-1):
    if windows[i] < windows[i+1]:
        count += 1
print("Number of windowed increases:", count)
