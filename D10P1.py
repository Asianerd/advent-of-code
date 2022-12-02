raw_data_string = """
.#..#
.....
#####
....#
...##
"""

asteroids = []

for y, line in enumerate(raw_data_string.strip().split('\n')):
    for x, c in enumerate(line):
        if c == "#":
            asteroids.append([x, y, []])

for x in asteroids:
    for c in asteroids:
        if x == c:
            continue
        if x[1] == c[1]:
            x[2].append(999999)
        elif x[0] == c[0]:
            x[2].append(888888)
        else:
            x[2].append(
                (c[1] - x[1]) / (c[0] - x[0])
            ) # y2 - y1 / x2 - x1
    x[2] = list(set(x[2]))

# [0.6666666666666666, 0, -0.0, -0.0, -0.0, -0.0, 0, -2.0, 0]
# 0.66, 0, -2

print(asteroids)

print(len(sorted(asteroids, key=lambda x:len(x[2]))[-1][2]))

final = []
for y in range(5):
    final.append([])
    for x in range(5):
        final[y].append(0)

for x in asteroids:
    final[x[1]][x[0]] = len(x[2])

print('\n'.join([''.join([('.' if n == 0 else str(n)) for n in i]) for i in final]))
