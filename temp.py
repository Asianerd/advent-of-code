from functools import reduce

inpt = []
with open("day11.txt") as f:
    for i in f.read().split("\n\n"):
        b = []
        c = i.split("\n")

        b.append([int(j) for j in c[1].split(":")[1].split(", ")]) # 0 = holding
        b.append(c[2].split(": ")[1].split(" = ")[1]) # 1 = modification
        b.append(int(c[3].split(" ")[-1])) # 2 = divisible test
        b.append((int(c[5][-1]), int(c[4][-1]))) # 3 = false, true monkeys
        b.append(0) # 4 = number times inspected

        inpt.append(b)

mod = reduce(lambda x, y: x * y, (i[2] for i in inpt))
for _ in range(10000):
    for monkey in inpt:
        monkey[-1] += len(monkey[0])

        for old in monkey[0]:
            # modified = eval(monkey[1].replace("old", str(old))) // 3 # PART 1
            modified = eval(monkey[1].replace("old", str(old))) % mod # PART 2
            inpt[monkey[3][modified % monkey[2] == 0]][0].append(modified)

        monkey[0] = []

a = sorted([i[-1] for i in inpt], reverse=True)
print(a[0] * a[1])