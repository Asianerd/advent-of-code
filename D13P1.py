raw_data_string = """
1002578
19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,751,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,431,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17
"""

starting_time = int(raw_data_string.strip().split("\n")[0])

raw_data = []
for x in raw_data_string.strip().split("\n")[-1].split(","):
    if x == 'x':
        continue
    raw_data.append(int(x))

data = []
for x in raw_data:
    _lowest_nearest = int(starting_time / x)

    final = []
    for item in range(_lowest_nearest, _lowest_nearest + 10):
        data.append([item * x, x])

data.sort(key=lambda x:x[0])
result = [x for x in data if (x[0] >= starting_time)][0]

print(result)

wait_time = result[0] - starting_time
score = wait_time * result[1]

print(score)
