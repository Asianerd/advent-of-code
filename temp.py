import re

inpt = []

with open('input.txt') as f:
    for line in f.readlines():
        inpt += [[*map(int, re.findall(r'-?\d+', line))]]

print(inpt)
pos = set()
y = 2000000

for row in inpt:

    sx, sy, bx, by = row

    dist = abs(sx - bx) + abs(sy - by)
    width = dist - abs(y - sy)

    if width < 0:
        continue

    for x in range(sx - width, sx + width + 1):
        pos.add(x)

for row in inpt:
    if row[3] == y:
        if row[2] in pos:
            pos.remove(row[2])

print(len(pos))


def get_ranges(y):
    a = []

    for row in inpt:
        sx, sy, bx, by = row

        dist = abs(sx - bx) + abs(sy - by)
        width = dist - abs(y - sy)

        if width < 0:
            continue
        
        a += [(sx - width, sx + width)]
    
    b = []
    for begin,end in sorted(a):
        if b and b[-1][1] >= begin - 1:
            b[-1][1] = max(b[-1][1], end)
        else:
            b.append([begin, end])
    
    return b

for i in range(4000000):
    x = get_ranges(i)
    if len(x) > 1:
        print(i + (x[0][1] + 1) * 4000000)
        break