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



def empty_rows(gmap):
    rows = []
    for i in range(len(gmap)):
        if all(c == '.' for c in gmap[i]):
            rows.append(i)
    return rows

def empty_cols(gmap):
    return empty_rows(transpose(gmap))

def down_distance(row1, row2, empty_rows):
    _row1 = min(row1, row2)
    _row2 = max(row1, row2)

    if row1 == row2:
        return 0
    d = 0
    for i in range(_row1 + 1, _row2):
        if i in empty_rows:
            d += 1000000
        else:
            d += 1
    return d + 1

def sum_star_dist(locs, erow, ecol):
    sum_dist = 0
    for i in range(len(locs)):
        for j in range(i + 1, len(locs)):
            l = locs[i]
            k = locs[j]
            sum_dist += down_distance(l[0], k[0], erow) + down_distance(l[1], k[1], ecol)
    return sum_dist

with open('input20.txt') as f:
    
    gmap = [list(row.strip()) for row in f.read().strip().split()]
    locs = star_locs(gmap)

    erow = set(empty_rows(gmap))
    ecol = set(empty_cols(gmap))

    print(sum_star_dist(locs, erow, ecol))
    