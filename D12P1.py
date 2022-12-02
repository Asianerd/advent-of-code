raw_data_string = """
<x=14, y=2, z=8>
<x=7, y=4, z=10>
<x=1, y=17, z=16>
<x=-4, y=-1, z=1>
"""

raw_data_string = """
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
"""

class Moon:
    collection = []
    
    def __init__(self, pos, vel=[0, 0, 0]):
        self.pos = pos
        self.vel = vel
        
        self.next_pos = list(self.pos)
    
    def data(self):
        return f'{self.pos}\t{self.vel}'
    
    def updateGravity(self):
        for x in Moon.collection:
            if self == x:
                continue
            if self.pos[0] > x.pos[0]:
                self.vel[0] -= 1
            elif self.pos[0] < x.pos[0]:
                self.vel[0] += 1
                
            if self.pos[1] > x.pos[1]:
                self.vel[1] -= 1
            elif self.pos[1] < x.pos[1]:
                self.vel[1] += 1
            
            if self.pos[2] > x.pos[2]:
                self.vel[2] -= 1
            elif self.pos[2] < x.pos[2]:
                self.vel[2] += 1

    def updatePosition(self):
        self.pos[0] += self.vel[0]
        self.pos[1] += self.vel[1]
        self.pos[2] += self.vel[2]
    
    @staticmethod
    def step():
        for x in Moon.collection:
            x.updateGravity()
        
        for x in Moon.collection:
            x.pos = list(x.next_pos)
        
        for x in Moon.collection:
            x.updatePosition()
    

for x in raw_data_string.strip().split('\n'):
    data = x.split(', ')
    Moon.collection.append(Moon(
        [
            int(data[0][3:]),
            int(data[1][2:]),
            int(data[2][2:-1])
        ]
    ))

for x in range(5):
    Moon.step()
    for x in Moon.collection:
        print(x.data())
    print('\n\n')
