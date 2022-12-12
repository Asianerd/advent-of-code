import math

raw_data_string = """
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""

class Step:
    content = 'abcdefghijklmnopqrstuvwxyz'
    steps = []
    
    def __init__(self, pos, height, t):
        if t == 0:
            self.height = Step.content.index(height)
        else:
            self.height = 0
        self.pos = pos
        
        self.step_type = t # 0 regular, 1 start, 2 end
        
        self.neighbours = []
    
    def fetch_neighbours(self):
        self.neighbours = [
            i for i in Step.steps if (math.sqrt(((self.pos[0] - i.pos[0]) ** 2) + ((self.pos[1] - i.pos[1]) ** 2)) == 1)
        ]
    
    def find_lowest(self):
        collection = sorted(self.neighbours, key=(lambda x:x.height))
        return [i for i in collection if self.height > i.height]

for x, line in enumerate(raw_data_string.strip().split('\n')):
    for y, c in enumerate(line):
        Step.steps.append(Step([x, y], c, 1 if c == 'S' else (2 if c == 'E' else 0)))


field = [[0 for i in range(5)] for ii in range(8)]
for x in Step.steps:
    x.fetch_neighbours()
    x.find_lowest()
    field[x.pos[1]][x.pos[0]] = len(x.neighbours)

for x in field:
    print(''.join([str(i) for i in x]))


    
    
