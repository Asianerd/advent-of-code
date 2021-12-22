raw_data_string = """
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
"""


class Point:
    points = []

    def __init__(self, position, is_on):
        self.position = position    # List of ints [x, y, z]
        self.on = is_on             # Bool

    @staticmethod
    def fetch_at(position):
        # result = [x for x in Point.points if x.position == position]
        # if len(result) >= 1:
        #     return True, result
        # return False
        return [x for x in Point.points if x.position == position][0]


for z in range(-50, 51):
    for y in range(-50, 51):
        for x in range(-50, 51):
            Point.points.append(Point([x, y, z], True))


def fetch_from_range_data(string):
    result = [int(x) for x in string.split("=")[-1].split("..")]
    return range(result[0], result[1])


for index, raw_data in enumerate(raw_data_string.strip().split("\n")):
    print(index)
    state = raw_data.split(" ")[0]
    range_data = raw_data.split(" ")[-1].split(",")
    x_range = fetch_from_range_data(range_data[0])
    y_range = fetch_from_range_data(range_data[1])
    z_range = fetch_from_range_data(range_data[2])
    for z in z_range:
        if z not in range(-50, 51):
            continue
        for y in y_range:
            if y not in range(-50, 51):
                continue
            for x in x_range:
                if x not in range(-50, 51):
                    continue
                print(f"{x}, {y}, {z}")
                point = Point.fetch_at([x, y, z])
                point.on = state

print(len([x for x in Point.points if x.on]))
