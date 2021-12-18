import math

raw_data_string = """
target area: x=137..171, y=-98..-73
"""

target_area = [
    range(137, 171 + 1),
    range(-98, -73 + 1)
    # range(20, 30 + 1),
    # range(-10, -5 + 1)
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

    any_success = False

    simulation_times = 0
    for iteration in range(250):
        simulation_times += 1

        position[0] += velocity[0]
        position[1] += velocity[1]

        if is_in_target(position):
            any_success = True

        if any_success:
            break

        # if not ((position[0] <= target_bottom_right[0]) and (position[1] >= target_bottom_right[1])):
        #     break

        if velocity[0] != 0:
            if velocity[0] > 0:
                velocity[0] -= 1
            elif velocity[0] < 0:
                velocity[0] += 1
        velocity[1] -= 1

        #print(f"Vel:{velocity} Pos:{position} BotR:{target_bottom_right}")

    return any_success


#print(simulate([6, 0]))

successful_velocities = []
simulation_range = (0, 300)
y_simulation_range = (-200, 300)
for y in range(y_simulation_range[0], y_simulation_range[1]):
    for x in range(simulation_range[0], simulation_range[1]):
        velocity = [x, y]
        result = simulate([x, y])
        if result:
            successful_velocities.append(result)
            print(f"{len(successful_velocities)}. {velocity} : Simulation success")

print(len(successful_velocities))