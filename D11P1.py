import math

raw_data_string = """
Monkey 0:
  Starting items: 56, 56, 92, 65, 71, 61, 79
  Operation: new = old * 7
  Test: divisible by 3
    If true: throw to monkey 3
    If false: throw to monkey 7

Monkey 1:
  Starting items: 61, 85
  Operation: new = old + 5
  Test: divisible by 11
    If true: throw to monkey 6
    If false: throw to monkey 4

Monkey 2:
  Starting items: 54, 96, 82, 78, 69
  Operation: new = old * old
  Test: divisible by 7
    If true: throw to monkey 0
    If false: throw to monkey 7

Monkey 3:
  Starting items: 57, 59, 65, 95
  Operation: new = old + 4
  Test: divisible by 2
    If true: throw to monkey 5
    If false: throw to monkey 1

Monkey 4:
  Starting items: 62, 67, 80
  Operation: new = old * 17
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 6

Monkey 5:
  Starting items: 91
  Operation: new = old + 7
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 4

Monkey 6:
  Starting items: 79, 83, 64, 52, 77, 56, 63, 92
  Operation: new = old + 6
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 7:
  Starting items: 50, 97, 76, 96, 80, 56
  Operation: new = old + 3
  Test: divisible by 13
    If true: throw to monkey 3
    If false: throw to monkey 5
"""

araw_data_string = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"""

class Monkey:
    monkeys = []
    
    def __init__(self, number, bags, divide, op, op_amount, destination):
        self.number = number
        
        self.bags = bags
        self.next_bags = []
        
        self.op = op # 0 add, 1 multiply, 2 square
        self.op_amount = op_amount
        
        self.test = divide
        self.destination = destination # [true, false]
        
        self.inspect_count = 0
    
    def evaluate_bags(self):
        new_bags = []
        for x in self.bags:
            self.inspect_count += 1
            
            if self.op == 0:
                # add
                new_bags.append(
                    x + self.op_amount
                )
            elif self.op == 1:
                # multiply
                new_bags.append(
                    x * self.op_amount
                )
            else:
                # square
                new_bags.append(
                    x * x
                )
        
        self.bags = []
        
        for x in new_bags:
            final = int(math.floor(x / 3))

            if (final % self.test) == 0:
                # if can be divided
                Monkey.monkeys[self.destination[0]].bags.append(final)
            else:
                # cannot be divided
                Monkey.monkeys[self.destination[1]].bags.append(final)
    
    @staticmethod
    def iterate():
        for x in Monkey.monkeys:
            x.evaluate_bags()
        
        # for x in Monkey.monkeys:
        #     x.bags = list(x.next_bags)
        
        # for x in Monkey.monkeys:
        #     x.next_bags = []

for monkey_index, x in enumerate([i.strip() for i in raw_data_string.strip().split('Monkey')][1:]):
    data = x.split('\n')
    starting = [int(n) for n in data[1][18:].split(', ')]
    op_statement = data[2][23:].split()
    if op_statement[0] == '*':
        if op_statement[1] == 'old':
            op_type = 2
            op_amount = 1
        else:
            op_type = 1
            op_amount = int(op_statement[1])
    else:
        op_type = 0
        op_amount = int(op_statement[1])
    
    test = int(data[3].split(' ')[-1])
    dest = [
        int(data[4].split(' ')[-1]),
        int(data[5].split(' ')[-1]),
    ]
    
    Monkey.monkeys.append(
        Monkey(
            monkey_index,
            starting,
            test,
            op_type,
            op_amount,
            dest        
        )
    )
    
for x in range(20):
    Monkey.iterate()

final = (sorted([i.inspect_count for i in Monkey.monkeys])[::-1])[:2]
print(final[0] * final[1])

# 67830
