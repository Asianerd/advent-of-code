raw_data_string = "5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2"

fishes = []
for x in raw_data_string.split(","):
    fishes.append(int(x))


def update():
    for x in range(len(fishes)):
        if fishes[x] <= 0:
            fishes[x] = 6
            fishes.append(8)
        else:
            fishes[x] -= 1


for i in range(256):
    print(i)
    update()

print(len(fishes))
# pain in the ass
