import math

raw_data_string = """
target area: x=137..171, y=-98..-73
"""

target_area = [
    range(137, 171 + 1),
    range(-98, -73 + 1)
]

target_bottom_right = [
    target_area[0][-1],
    target_area[1][-1]
]

print(target_bottom_right)


def is_in_target(position):
    return (position[0] in target_area[0]) and (position[1] in target_area[1])


def simulate(velocity):
    position = [0, 0]

    highest_y = 0
    any_success = False
    while not any_success:
        if velocity[0] != 0:
            if velocity[0] > 0:
                velocity[0] -= 1
            else:
                velocity[0] += 1
        velocity[1] -= 1

        position[0] += velocity[0]
        position[1] += velocity[1]
        if position[1] >= highest_y:
            highest_y = position[1]

        if is_in_target(position):
            any_success = True

        if any_success:
            break

        if not ((position[0] <= target_bottom_right[0]) and (position[1] >= target_bottom_right[1])):
            break

        #print(f"Vel:{velocity} Pos:{position} BotR:{target_bottom_right}")

    return [any_success, highest_y]


#print(simulate([]))

successful_velocities = []
simulation_range = (0, 500)
for y in range(simulation_range[0], simulation_range[1]):
    for x in range(simulation_range[0], simulation_range[1]):
        velocity = [x, y]
        result = simulate([x, y])
        if result[0]:
            successful_velocities.append(result)
            print(f"{velocity} : Simulation success")

for x in successful_velocities:
    print(x)

"""
0 - 500
[True, 630]
[True, 630]
[True, 2628]
[True, 2628]



"""
