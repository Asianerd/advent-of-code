raw_data_string = """
1002578
19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,751,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,431,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17
"""

# raw_data_string = """
# 1002578
# 7,13,x,x,59,x,31,19
# """

upper_check = 1000000000000000000
#             100000000000000
#             538703333547789

data = []
for i, x in enumerate(raw_data_string.strip().split("\n")[-1].split(",")):
    if x == 'x':
        continue
    data.append([int(x), i, range(0, upper_check, int(x))])
    # [increment, index, range]

increment = -1
while True:
    print(f"i : {increment}")
    increment += 1
    found = True
    for i in data:
        if not ((increment + i[1]) in i[2]):
            found = False
            break
    if found:
        print(f'\n\nEarliest timestamp : {increment}')
        break

    if increment > upper_check:
        print(f'Surpassed upper_check')
        break

# Answer : 538703333547789
