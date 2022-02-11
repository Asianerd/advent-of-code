raw_data_string = """
6,4,12,1,20,0,16
"""

numbers = []
for x in raw_data_string.strip().split(","):
    numbers.append(int(x))

while len(numbers) < 2020:
    occurances = [[i, index] for index, i in enumerate(numbers) if i == numbers[-1]]
    if len(occurances) == 1:
        numbers.append(0)
    else:
        _first_previous = occurances[-1]
        _second_previous = occurances[-2]
        score = _first_previous[1] - _second_previous[1]
        numbers.append(score)

print(f"{len(numbers)} : {numbers[-1]} : {numbers}")
