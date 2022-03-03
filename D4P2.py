raw_data_string = """
265275-781584
"""

upper, lower = [int(x) for x in raw_data_string.strip().split("-")]

amount = 0


def count_concurrent_chars(string):
    result = []
    previous_char = 0
    for current_index, current_char in enumerate(string):
        if previous_char != current_char:
            result.append([current_char, 1])
            previous_char = current_char
        else:
            result[-1][1] += 1
    return result


for iteration in range(upper, lower):
    _string = str(iteration)
    result = False

    # char count
    count = count_concurrent_chars(_string)
    if len([i for i in count if i[1] == 2]) < 1:
        result = False
        continue
    else:
        result = True

    # decreasing check
    for index, char in enumerate(_string[:-1]):
        if int(char) > int(_string[index+1]):
            result = False
            break

    if not result:
        continue

    print(_string)
    amount += 1

print(amount)
#373 - too low
#626