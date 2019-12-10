test_orbits = '''COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN'''

def parse_rel_map(raw_rels):
    rel_list = [orbit.split(')') for orbit in raw_rels.split('\n')]
    return {rel[1]: rel[0] for rel in rel_list}

def num_rels(obj, rel_map):
    # COM orbits nothing
    if obj == 'COM':
        return 0

    if obj not in rel_map:
        return 0

    return 1 + num_rels(rel_map[obj], rel_map)

test_rel_map = parse_rel_map(test_orbits)

def test():

    assert num_rels('COM', test_rel_map) == 0
    assert num_rels('D', test_rel_map) == 3
    assert num_rels('L', test_rel_map) == 7

test()

print(sum(num_rels(obj, test_rel_map) for obj in test_rel_map))

rel_map = parse_rel_map(open('input.dat').read().strip())
print(sum(num_rels(obj, rel_map) for obj in rel_map))

def calc_path(obj, rel_map, stops):
    steps, path = 0, {}

    while True:
        if obj not in rel_map or obj in stops:
            break

        obj = rel_map[obj]
        path[obj] = steps
        steps += 1
    return path

mpath = calc_path('YOU', rel_map, {})
spath = calc_path('SAN', rel_map, mpath)
i = max(spath.items(), key=lambda obj: obj[1])

print(mpath, spath)
print(mpath[i[0]] + i[1])