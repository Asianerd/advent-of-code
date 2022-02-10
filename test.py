binary = """000000000000000000000000000000X1101X"""
expanded = []
amount = len([x for x in binary if x == 'X'])

for x in range(2**amount):
    bin_string = str(bin(x))[2:].rjust(amount, '0')
    final = [i for i in binary]
    incremented_index = 0
    for index, item in enumerate(binary):
        if item == 'X':
            final[index] = bin_string[incremented_index]
            incremented_index += 1
    expanded.append(int(''.join(final), 2))

print(expanded)
