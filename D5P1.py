raw_data_string = """
3,225,1,225,6,6,1100,1,238,225,104,0,1102,17,65,225,102,21,95,224,1001,224,-1869,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,101,43,14,224,1001,224,-108,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,57,94,225,1101,57,67,225,1,217,66,224,101,-141,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1102,64,34,225,1101,89,59,225,1102,58,94,225,1002,125,27,224,101,-2106,224,224,4,224,102,8,223,223,1001,224,5,224,1,224,223,223,1102,78,65,225,1001,91,63,224,101,-127,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,7,19,224,1001,224,-133,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,2,61,100,224,101,-5358,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,19,55,224,101,-74,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1101,74,68,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,344,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,359,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,374,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,389,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,404,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,419,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,434,101,1,223,223,1108,677,677,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,464,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,479,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,494,101,1,223,223,107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,539,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,554,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1007,677,677,224,102,2,223,223,1005,224,584,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,599,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,614,101,1,223,223,108,677,226,224,1002,223,2,223,1005,224,629,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,644,101,1,223,223,1007,677,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,226,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226
"""

instructions = [int(x) for x in raw_data_string.strip().split(",")]

# for index, x in enumerate(instructions):
#     print(f'{index}. {x}')


def get_parameter(parent, index, parameter):
    modes = str(parent).zfill(5)[:3][::-1].zfill(3)
    if modes[2] == '1':
        print(f'\t\tMode 3 isnt 0 : @{execution_index}')
    if index >= len(modes):
        mode = 0
    else:
        mode = int(modes[index])
    #print(f'{parent} : {modes}')
    if mode == 0:
        return instructions[parameter]
    else:
        return parameter


execution_index = 0
run = True
input_number = 1
monitored_value = 125
monitor_value = True

while run:
    current = instructions[execution_index]
    command = int(str(current)[-2:])
    if current == 99:
        print(f"End at {execution_index}")
        break

    #print(f'current:{current}')

    if command == 1:  # addition
        a = get_parameter(current, 0, instructions[execution_index + 1])
        b = get_parameter(current, 1, instructions[execution_index + 2])
        target = instructions[execution_index + 3]
        if monitor_value:
            if target == monitored_value:
                print(f"+@{execution_index}\t{instructions[monitored_value]}\t{a}\t{b}\t{target}")
        #print(a, b, target)
        # if execution_index == 106:
        #     print(a,b,target)
        instructions[target] = a + b
        execution_index += 3

    if command == 2:  # multiplication
        a = get_parameter(current, 0, instructions[execution_index + 1])
        b = get_parameter(current, 1, instructions[execution_index + 2])
        target = instructions[execution_index + 3]
        if monitor_value:
            if target == monitored_value:
                print(f"*@{execution_index}\t{instructions[monitored_value]}\t{a}\t{b}\t{target}")
        instructions[target] = a * b
        execution_index += 3

    if command == 3:  # write {input} at {parameter}
        #p = get_parameter(current, 0, instructions[execution_index + 1])
        p = instructions[execution_index + 1]
        # change this to positional parameter only if it program doesnt work
        instructions[p] = input_number
        execution_index += 1

    if command == 4:  # print value at {parameter}
        out = instructions[instructions[execution_index + 1]]
        target = instructions[execution_index + 1]
        print(f'\tOut : {out}(t:{target})')
        # if out != 0:
        #     print(f'############# Output isnt 0; execution_index at {execution_index}')
        execution_index += 1

    execution_index += 1


print(instructions)
# 84084  too low
# 359667609   too high
# 15259545 correct
