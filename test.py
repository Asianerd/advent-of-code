collection = [12, 3, 4, 1, 3, 4, 1, 2, 4, 6, 7, 3, 7, 6, 34, 2, 4, 5, 6, 2556, 34, 2, 1, 414, 23, 5, 46]
collection.sort()

print(collection)

for x in [i for i in collection if (i % 2 == 0)]:
    x += 1

print(collection)