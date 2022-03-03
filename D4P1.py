raw_data_string = """
265275-781584
"""

upper, lower = [int(x) for x in raw_data_string.strip().split("-")]

amount = 0

for iteration in range(upper, lower):
    _string = str(iteration)
    result = False
    for x in range(0, len(_string) - 1):
        if _string[x] == _string[x+1]:
            result = True
            break
    for index, char in enumerate(_string[:-1]):
        if int(char) > int(_string[index+1]):
            result = False
            break
    if not result:
        continue

    print(_string)
    amount += 1

print(amount)