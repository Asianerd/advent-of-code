import math

raw_data_string = """
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"""


class Point:
    points = []
    head = None
    start = None
    end = None
    path = []
    found = False

    def __init__(self, position, risk):
        self.position = position
        self.risk = risk
        self.neighbours = []

        if position == [0, 0]:
            Point.start = self
        elif position == [9, 9]:
            Point.end = self

        self.visited = False

    def fetch_neighbours(self):
        self.neighbours = [x for x in Point.points if (x.distance(self.position) <= 1) and (x.position != self.position)]
        self.neighbours.sort(key=lambda x:x.risk)

    def distance(self, pos2):
        return math.sqrt(
            math.pow(self.position[0] - pos2[0], 2) +
            math.pow(self.position[1] - pos2[1], 2)
        )

    @staticmethod
    def find_path():
        neighbours = [x for x in Point.head.neighbours if not x.visited]
        Point.path.append(neighbours[0])
        Point.head.visited = True
        Point.head = neighbours[0]
        if Point.head == Point.end:
            Point.found = True


for y, column in enumerate(raw_data_string.strip().split("\n")):
    for x, item in enumerate(column):
        Point.points.append(Point([x, y], item))

for x in Point.points:
    x.fetch_neighbours()

Point.path.append(Point.start)
Point.head = Point.start
while not Point.found:
    Point.find_path()

for x in Point.path:
    print(f"{x.risk} {x.position}")
print(sum([x.risk for x in Point.path]))
