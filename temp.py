import networkx as nx

def parser(filename: str):
    with open(filename, 'r') as file:
        loc_dict = {complex(x, y): v for x, row in enumerate(file.read().splitlines()) for y, v in enumerate(row)}
        start, end = next(i for i, v in loc_dict.items() if v == 'S'), next(i for i, v in loc_dict.items() if v == 'E')
        loc_dict[start] = 'a'
        loc_dict[end] = 'z'
        return loc_dict, start, end


def part_a_solver(loc_dict: dict[complex, str], start: complex, end: complex):
    def get_neighbours(z: complex):
        return (z + d for d in (1, 1j, -1, -1j) if z + d in loc_dict and ord(loc_dict[z + d]) <= ord(loc_dict[z]) + 1)
    graph = nx.DiGraph([(pt, npt, {'weight': 1}) for pt in loc_dict for npt in get_neighbours(pt)])
    return nx.dijkstra_path_length(graph, start, end, weight='weight')


def part_b_solver(loc_dict: dict[complex, str], start: complex, end: complex):
    a_loc = tuple(i for i, v in loc_dict.items() if v == 'a')
    def get_neighbours(z: complex):
        return tuple(z + d for d in (1, 1j, -1, -1j) if z + d in loc_dict and ord(loc_dict[z + d]) >= ord(loc_dict[z]) - 1) + \
            (a_loc if loc_dict[z] == 'a' else tuple())

    graph = nx.DiGraph([(pt, npt, {'weight': 1}) for pt in loc_dict for npt in get_neighbours(pt)])
    return nx.dijkstra_path_length(graph, end, start, weight='weight') - 1

loc, start, end = parser('input.txt')
print(part_a_solver(loc, start, end))
print(part_b_solver(loc, start, end))

