from collections import defaultdict
from math import atan, degrees, fabs, sqrt

def collect_points(data):
    points = []
    for y, row in enumerate(data.strip().split('\n')):
        for x, value in enumerate(row):
            if value == '#':
                points.append((x, y))
        
    return points

def dist(p, q):
    return sqrt((q[0] - p[0])**2 + (q[1] - p[1])**2)

def slope(p, q):
    if p[0] == q[0]:
        return None
    return (q[1] - p[1]) / (q[0] - p[0])

def dir_single(p, q):
    if p < q:
        return 1
    elif p > q:
        return -1
    else:
        return 0

def dir_bi(p, q):
    return (dir_single(p[0], q[0]), dir_single(p[1], q[1]))

def calc_slope(p, q):
    return slope(p, q), dir_bi(p, q)

def calc_slopes(p, points):
    for q in points:
        if q != p:
            yield (q, calc_slope(p, q))

def calc_degrees(slope):
    if slope[0] is None:
        return 180. if slope[1][1] == 1 else 0.

    if slope[1][1] == 0:
        return 90. if slope[1][0] == 1 else 270.

    d = degrees(atan(slope[0]))
    if slope[1][0] == 1:
        d = d + 90.
    elif slope[1][0] == -1:
        d = d + 270.
    return d
    
def best_location(points):
    rating = {}
    for o in points:
        d = defaultdict(list)
        for p, slope in calc_slopes(o, points):
            d[slope].append(p)
        rating[o] = len(d)

    return max(rating.items(), key=lambda i: i[1])

def test():
    points = collect_points('''
.#..#
.....
#####
....#
...##''')
    assert best_location(points) == ((3, 4), 8)

    points = collect_points('''
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####''')
    assert best_location(points) == ((5, 8), 33)

    points = collect_points('''
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..''')
    assert best_location(points) == ((6, 3), 41)

    points = collect_points('''
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##''')
    assert best_location(points) == ((11, 13), 210)

test()

points = collect_points('''
#...##.####.#.......#.##..##.#.
#.##.#..#..#...##..##.##.#.....
#..#####.#......#..#....#.###.#
...#.#.#...#..#.....#..#..#.#..
.#.....##..#...#..#.#...##.....
##.....#..........##..#......##
.##..##.#.#....##..##.......#..
#.##.##....###..#...##...##....
##.#.#............##..#...##..#
###..##.###.....#.##...####....
...##..#...##...##..#.#..#...#.
..#.#.##.#.#.#####.#....####.#.
#......###.##....#...#...#...##
.....#...#.#.#.#....#...#......
#..#.#.#..#....#..#...#..#..##.
#.....#..##.....#...###..#..#.#
.....####.#..#...##..#..#..#..#
..#.....#.#........#.#.##..####
.#.....##..#.##.....#...###....
###.###....#..#..#.....#####...
#..##.##..##.#.#....#.#......#.
.#....#.##..#.#.#.......##.....
##.##...#...#....###.#....#....
.....#.######.#.#..#..#.#.....#
.#..#.##.#....#.##..#.#...##..#
.##.###..#..#..#.###...#####.#.
#...#...........#.....#.......#
#....##.#.#..##...#..####...#..
#.####......#####.....#.##..#..
.#...#....#...##..##.#.#......#
#..###.....##.#.......#.##...##
''')

def vaporized(p, points):
    d = defaultdict(list)
    for q, slope in calc_slopes(p, points):
        d[slope].append(q)

    v = []
    for slope in sorted(d.keys(), key=calc_degrees):
        v.append(sorted(d[slope], key=lambda q: dist(p, q))[0])
    return v

def nth_vaporized(p, points, n):
    vi = 0
    while len(points) != 1:
        for v in vaporized(loc, points):
            points.remove(v)
            vi += 1
            print(f'The {vi}st asteroid to be vaporized is at {v}')
            if vi == n:
                return v

loc, _ = best_location(points)
print(loc)
x, y = nth_vaporized(loc, points, 200)
print(x * 100 + y)
