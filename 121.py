raw_data_string = """
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"""
"""
pg-CH
pg-yd
yd-start
fe-hv
bi-CH
CH-yd
end-bi
fe-RY
ng-CH
fe-CH
ng-pg
hv-FL
FL-fe
hv-pg
bi-hv
CH-end
hv-ng
yd-ng
pg-fe
start-ng
end-FL
fe-bi
FL-ks
pg-start
"""


def is_big_cave(cave_name):
    return str(cave_name)[0].isupper()


caves = {}
for string in raw_data_string.strip().split("\n"):
    cave_data = string.split("-")
    for x in cave_data:
        if not (x in caves):
            caves[x] = []
    caves[cave_data[0]].append(cave_data[1])
    caves[cave_data[1]].append(cave_data[0])

paths = {}


def visit(next_cave, path):
    final = []

    new_path = list(path).copy()
    new_path.append(next_cave)
    if next_cave == 'end':
        return new_path
    for cave in caves[next_cave]:
        if cave == 'start':
            continue

        if (str(cave).isupper()) or not (next_cave in path):
            result = visit(cave, new_path)
            final.append(result)
    return final


def remove_empty(collection):
    final = []
    for x in collection.copy():
        if x:
            remove_empty(x)

final_result = visit('start', [])
print(final_result)
