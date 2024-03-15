from collections import deque
import heapq

def index(box, coord):
    return box[coord[0]][coord[1]]

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

RIGHT2 = (0, 2)
DOWN2 = (2, 0)
UP2 = (-2, 0)
LEFT2 = (0, -2)

RIGHT3 = (0, 3)
DOWN3 = (3, 0)
UP3 = (-3, 0)
LEFT3 = (0, -3)
def mul(coord, a):
    return (coord[0] * a, coord[1] * a)

LEFTS =[ mul(LEFT, i) for i in range(4, 10 + 1)]
UPS =[ mul(UP, i) for i in range(4, 10 + 1)]
DOWNS =[ mul(DOWN, i) for i in range(4, 10 + 1)]
RIGHTS =[ mul(RIGHT, i) for i in range(4, 10 + 1)]
DIRS = LEFTS + UPS + DOWNS + RIGHTS


def nega(coord):
    return (-coord[0], -coord[1])

def is_opp(c1, c2):
    return is_par(nega(c1), c2)
    raise ValueError

def same_sign(a, b):
    return (a > 0 and b > 0) or (a < 0 and b < 0)

def is_par(c1, c2):
    return (c1[0] == c2[0] == 0 and same_sign(c1[1], c2[1])) or (c1[1] == c2[1] == 0 and same_sign(c1[0], c2[0]))

def box_cost(box, t, dir):
    if is_par(dir, RIGHT):
        return sum(index(box, (t[0], t[1] + k)) for k in range(1, dir[1] + 1))
    elif is_par(dir, DOWN):
        return sum(index(box, (t[0] + k, t[1])) for k in range(1, dir[0] + 1))
    elif is_par(dir, LEFT):
        return sum(index(box, (t[0], t[1] + k)) for k in range(-1,  dir[1] - 1, -1))
    elif is_par(dir, UP):
        return sum(index(box, (t[0] + k, t[1])) for k in range(-1,  dir[0] - 1, -1))
    print(dir)
    raise ValueError

def box_fill(box, t, dir):
    # print('filling', t, dir)
    if is_par(dir, RIGHT):
        # print('right')
        for k in range(1, dir[1] + 1):
            # print(t[0], t[1] + k)
            box[t[0]][t[1] + k] = '>'
    elif is_par(dir, DOWN):
        # print('down')
        for k in range(1, dir[0] + 1):
            box[t[0] + k][t[1]] = 'v'
            # print(t[0] + k, t[1])
    elif is_par(dir, LEFT):
        # print('left')
        for k in range(-1, dir[1] - 1, -1):
            # print(t[0], t[1] + k)
            box[t[0]][t[1] + k] = '<'
    elif is_par(dir, UP):
        # print('up', len(range(-1, dir[0] - 1, -1)))
        for k in range(-1, dir[0] - 1, -1):
            # print(t[0] + k, t[1])
            box[t[0] + k][t[1]] = '^'
    # print('done filling')

str_dir = {
        UP: 'UP',
        DOWN: 'DOWN',
        LEFT: 'LEFT',
        RIGHT: 'RIGHT',
         UP2: 'UP2',
        DOWN2: 'DOWN2',
        LEFT2: 'LEFT2',
        RIGHT2: 'RIGHT2',
         UP3: 'UP3',
        DOWN3: 'DOWN3',
        LEFT3: 'LEFT3',
        RIGHT3: 'RIGHT3',
    }
def dir_str(dir):
    return str_dir[dir]

def coord_str(coord):
    return str(coord[0]) + ' ' + dir_str(coord[1])

with open('input17.txt') as f:
    a = [[int(c) for c in row] for row in f.read().strip().split()]
    # print('dims', len(a), len(a[0]))

    box = a

    distance = {((0, 0), RIGHT): 0, ((0, 0), DOWN): 0}
    
    frontier = [(0, ((0, 0), RIGHT)), (0, ((0, 0), DOWN))]
    visited = set()
    queued = set()
    parent = {}
    cn =(len(box) - 1, len(box[0]) - 1)
    while frontier:
        # print('queue', [(c, coord_str(s)) for c, s in frontier])
        c, t = heapq.heappop(frontier)
        if t in visited:
            continue
        # print('t', coord_str(t), c, distance[t])
        t_i = t[0]
        if t_i == cn:
            break
        t_d = t[1]

        next_dirs = ((add(t_i, d), d) for d in DIRS)

        f_dirs = [(pos, dir) for pos, dir in next_dirs if 0 <= pos[0] < len(box) and 0 <= pos[1] < len(box[0]) and (not is_par(t_d, dir)) and (not is_opp(t_d, dir))]

        # print('t', coord_str(t))
        # print('f_dirs', [coord_str(d) for d in f_dirs])
        f_costs = [box_cost(box, t_i, dir) for _, dir in f_dirs]
        # print('f_costs', f_costs)
        
        visited.add(t)
        for fd, dir_cost in zip(f_dirs, f_costs):
            # print('considering', coord_str(fd), 'at cost', dir_cost)
            # print('visited', fd in visited, 'distanced', fd in distance)
            cost = distance[t] + dir_cost
            if fd not in visited and fd not in queued:
                # print('added distance', cost)
                distance[fd] = cost
                heapq.heappush(frontier, (cost, fd))
                queued.add(fd)
                parent[fd] = t
            elif fd in distance and distance[fd] > cost:
                # print('updated distance', cost)
                # print('cost to', coord_str(fd), 'updated from', distance[fd], 'to', cost)
                distance[fd] = cost
                heapq.heappush(frontier, (cost, fd))
                parent[fd] = t
    


    min_dist = 100 ** 10
    min_dir = None
    for d in DIRS:
        corner = (cn, d)
        if corner in distance:
            dist = distance[corner]
            if min_dist > dist:
                min_dist = dist
                min_dir = d
            
    
    min_corner = (cn, min_dir)
    # print('min_corner', coord_str(min_corner))

    path = []
    # print('path')
    # print(cn, end=' ')
    p = min_corner
    path.append(p)
    while p[0] != (0, 0):
        p = parent[p]
        # print(p[0], end=' ')
        path.append(p)
    print()
    path = list(reversed(path))

    # print('path', [coord_str(i) for i in path])

    copy_box = [list(row) for row in box]

    for i in range(len(path) - 1):
        # print('step', path[i])
        # print('cost', box_cost(box, path[i][0], path[i + 1][1]))
        box_fill(copy_box, path[i][0], path[i + 1][1])
    
    print('\n'.join(''.join(str(i) for i in row) for row in copy_box))

    # for a in DIRS:
    #     for b in DIRS:
    #         print(dir_str(a), dir_str(b), is_opp(a, b), is_par(a, b))
    
    print(min_dist)
            
        