import math
"""
11111
19991
19191
19991
11111
"""
raw_data_string = """
7313511551
3724855867
2374331571
4438213437
6511566287
6727245532
3736868662
2348138263
2417483121
8812617112
"""


def display_dumbos():
    print()
    amount = int(math.sqrt(len(Dumbo.dumbos)))
    for y in range(amount):
        dumbos = []
        for x in range(amount):
            dumbos.append([i.energy for i in Dumbo.dumbos if i.position == [x, y]][0])
        print(''.join([str(x) for x in dumbos]))


class Dumbo:
    dumbos = []
    total_flashes = 0

    def __init__(self, energy, position):
        self.energy = energy
        self.position = position
        self.neighbours = []
        self.has_flashed = False

    @staticmethod
    def static_update():
        for x in Dumbo.dumbos:
            x.update()
        while len([x for x in Dumbo.dumbos if x.energy > 9]) != 0:
            for item in [i for i in Dumbo.dumbos if i.energy > 9]:
                item.has_flashed = True
                item.energy = 0
                Dumbo.total_flashes += 1
                for n in item.neighbours:
                    if n.has_flashed:
                        continue
                    n.energy += 1

    def update(self):
        self.has_flashed = False
        self.affect_energy(1)

    def secondary_update(self):
        pass

    def tertiary_update(self):
        pass

    def affect_energy(self, amount):
        self.energy += amount

    def fetch_neighbours(self):
        min_distance = math.sqrt(2)
        self.neighbours = [x for x in Dumbo.dumbos if (x.distance(self.position) <= min_distance) and (x.position != self.position)]

    def distance(self, position2):
        return math.sqrt(
            math.pow(self.position[0] - position2[0], 2) +
            math.pow(self.position[1] - position2[1], 2)
        )


for y, column in enumerate(raw_data_string.strip().split("\n")):
    for x, item in enumerate(column):
        Dumbo.dumbos.append(Dumbo(int(item), [x, y]))

for x in Dumbo.dumbos:
    x.fetch_neighbours()

display_dumbos()
for iteration in range(100):
    Dumbo.static_update()
    display_dumbos()
print(Dumbo.total_flashes)
# 1635?
