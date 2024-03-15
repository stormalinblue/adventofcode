import math

def expand_down(gmap):
    new_rows = []
    for row in gmap:
        if all(c == '.' for c in row):
            new_rows.append(row)
            new_rows.append(row)
        else:
            new_rows.append(row)
    return new_rows

def transpose(gmap):
    return [[row[i] for row in gmap] for i in range(len(gmap[0]))]

def expand(gmap):
    return expand_down(transpose(expand_down(transpose(gmap))))

def print_gmap(gmap):
    return '\n'.join(''.join(row) for row in gmap)

def manhattan_distance(coord1, coord2):
    return abs(coord1[0] - coord2[0]) + abs(coord1[1] - coord2[1])

def star_locs(gmap):
    locs = []
    for i in range(len(gmap)):
        for j in range(len(gmap[0])):
            if gmap[i][j] == '#':
                locs.append((i, j))
    return locs

def min_star_dist(locs):
    min_dist = math.inf
    for i in range(len(locs)):
        for j in range(i + 1, len(locs)):
            min_dist = min(min_dist, manhattan_distance(locs[i], locs[j]))
    return min_dist

def sum_star_dist(locs):
    sum_dist = 0
    for i in range(len(locs)):
        for j in range(i + 1, len(locs)):
            sum_dist += manhattan_distance(locs[i], locs[j])
    return sum_dist

with open('input20.txt') as f:
    
    gmap = [list(row.strip()) for row in f.read().strip().split()]
    print(print_gmap(gmap))
    ex_map = expand(gmap)
    locs = star_locs(ex_map)
    print(locs)
    print(sum_star_dist(locs))

    