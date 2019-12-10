def generate_path(path, start_position=(0, 0)):
    position = start_position

    for item in path:
        direction = item[0]
        distance = int(item[1:])

        calculate_next = {
            'R': lambda position: (position[0] + 1, position[1]),
            'L': lambda position: (position[0] - 1, position[1]),
            'U': lambda position: (position[0], position[1] + 1),
            'D': lambda position: (position[0], position[1] - 1),
        }.get(direction)

        for _ in range(distance):
            position = calculate_next(position)
            yield position

def apply_path(plane, path, mark='.'):
    intersections = []

    for position in path:
        if position in plane and plane[position] != mark:
            intersections.append(position)
        
        plane[position] = mark

    return intersections

def manhattan_disance(p, q):
    return abs(p[0] - q[0]) + abs(p[1] - q[1])

def find_closest_intersection(paths, start_position=(0, 0)):
    plane = {}
    intersections = set()
    for i, path in enumerate(paths):
        local_intersections = set(apply_path(plane, generate_path(path.split(',')), mark=i))
        intersections = intersections.union(local_intersections)

    distance = lambda p: manhattan_disance(p, start_position)
    closest_intersection = min(intersections, key=distance)
    return distance(closest_intersection)

def find_optimal_intersection(paths, start_position=(0, 0)):
    plane = {}
    intersections = set()
    for i, path in enumerate(paths):
        local_intersections = set(apply_path(plane, generate_path(path.split(',')), mark=i))       
        intersections = intersections.union(local_intersections)

    def steps_count(p):
        count = 0
        for path in paths:
            for i, position in enumerate(generate_path(path.split(','), start_position)):
                if p == position:
                    count += i + 1
                    break
        return count

    closest_intersection = min(intersections, key=steps_count)
    return steps_count(closest_intersection)


def test():
    assert find_closest_intersection(['R8,U5,L5,D3', 'U7,R6,D4,L4']) == 6
    assert find_closest_intersection(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83']) == 159
    assert find_closest_intersection(['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51', 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7']) == 135
    
    assert find_optimal_intersection(['R8,U5,L5,D3', 'U7,R6,D4,L4']) == 30
    assert find_optimal_intersection(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83']) == 610
    assert find_optimal_intersection(['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51', 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7']) == 410 

test()

with open('input.dat') as fp:
    paths = fp.read().strip().split('\n')
print(find_closest_intersection(paths))
print(find_optimal_intersection(paths))