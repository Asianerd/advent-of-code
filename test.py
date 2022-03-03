def reduce(iterable):
    result = ''
    for x in iterable:
        if type(x) == list:
            result += reduce(x)
        else:
            result += str(x)
    return result


print(reduce([0, [1, [0, [1, 0, 1]]], [[[0, 1], 0, [1, [0, [1, 0]]]], 1], 0]))
