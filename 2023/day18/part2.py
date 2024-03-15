RIGHT = (1, 0)
DOWN = (0, -1)
UP = (0, 1)
LEFT = (-1, 0)

def index(box, coord):
    return box[coord[0]][coord[1]]

def set_index(box, coord, val):
    # print(coord)
    box[coord[0]][coord[1]] = val

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

str_dir = {
        UP: 'UP',
        DOWN: 'DOWN',
        LEFT: 'LEFT',
        RIGHT: 'RIGHT',
    }
def dir_str(dir):
    return str_dir[dir]


def nega(coord):
    return (-coord[0], -coord[1])

def mul(coord, val):
    return (coord[0] * val, coord[1] * val)

def connected(a, b):
    return abs(sum(add(a, nega(b)))) == 1

def wall_area(walls):
    # print(walls)
    if not walls:
        return 0
    
    s = 0
    up_in = False
    down_in = False
    prev_inside = False
    inside_start = None
    for pos, dir in walls:

        if dir == '|':
            up_in = not up_in
            down_in = not down_in
        elif dir == '-':
            pass
        elif dir == '7':
            down_in = not down_in
        elif dir == 'L':
            up_in = not up_in
        elif dir == 'J':
            up_in = not up_in
        elif dir == 'F':
            down_in = not down_in

        incr = 0
        inside = up_in or down_in
        if inside == False and inside != prev_inside:
            # print('switch on', pos, dir)
            incr = pos[1] - inside_start + 1
            
        if inside == True and inside != prev_inside:
            inside_start = pos[1]
        prev_inside = inside
        # print('incr is', incr)
        s += incr
    return s


def paint(rows, cols, positions):
    board = [['.' for _ in range(cols)] for _ in range(rows)]

    for pos, sym in positions:
        set_index(board, pos, sym)
    
    return '\n'.join((''.join(row) for row in board))
    

def corner_areas(corners):
    if not corners:
        return 0
    first_corner = corners[0]
    area = 0
    for i in range(len(corners) - 1):
        corner1 = corners[i]
        corner2 = corners[i + 1]

        vec1 = add(nega(first_corner), corner1)
        vec2 = add(nega(first_corner), corner2)

        area += vec1[0] * vec2[1] - vec1[1] * vec2[0]
    return abs(area) // 2

with open('input18.txt') as f:
    moves = []

    dig_dir = {'0': 'R', '1': 'D', '2': 'L', '3': 'U'}
    for line in (l.strip() for l in f.read().strip().split('\n')):
        # print(line)
        d, l, c = line.split()

        col = c[2:-1]
        d = dig_dir[col[-1]]
        l = int(col[:-1], base=16)

        moves.append((d, l))

    # for line in (l.strip() for l in f.read().strip().split('\n')):
    #     # print(line)
    #     d, l, c = line.split()

    #     dir = d
    #     l = int(l)
    #     col = c[2:-1]
    #     moves.append((dir, l))
    
    move_syms = []
    prev_dir = moves[-1][0]
    for (d, _) in moves:
        dirmap = {
            ('D', 'L'): 'J',
            ('L', 'D'): 'F',
            ('R', 'D'): '7',
            ('D', 'R'): 'L',
            ('U', 'L'): '7',
            ('L', 'U'): 'L',
            ('R', 'U'): 'J',
            ('U', 'R'): 'F',
        }
        sym = dirmap[(prev_dir, d)]
        # print(sym)
        move_syms.append(sym)
        prev_dir = d
    

    
    positions = []
    cur_pos = (0, 0)
    # corners = []
    prev_sym = move_syms[-1]
    for (d, l), sym in zip(moves, move_syms):
        match d:
            case 'R':
                dir = RIGHT
            case 'L':
                dir = LEFT
            case 'U':
                dir = UP
            case 'D':
                dir = DOWN
        positions.append(cur_pos)
        # corners.append(add(cur_pos, corner_offset(prev_sym, sym)))
        cur_pos = add(cur_pos, mul(dir, l))
        prev_sym = sym


    
    

    min_row = min(i for (i, _) in positions)
    min_col = min(i for (_, i) in positions)
    max_row = max(i for (i, _) in positions)
    max_col = max(i for (_, i) in positions)
    rows = max_row - min_row + 1
    cols = max_col - min_col + 1

    print("dimensions", (max_row - min_row + 1), (max_col - min_col + 1))

    offset = nega((min_row, min_col))
    print('offset', offset)
    o_positions = [add(offset, pos) for pos in positions]

    assume_inner_corners = []
    assume_outer_corners = []
    prev_sym = move_syms[-1]
    for corner, sym in zip(o_positions, move_syms):
        match prev_sym:
            case 'L':
                # print('case L')
                match sym:
                    # To the right
                    case 'J':
                        assume_inner_corners.append(add((0, 1), corner))
                        assume_outer_corners.append(add((1, 0), corner))
                    case '7':
                        assume_inner_corners.append(add((1, 1), corner))
                        assume_outer_corners.append(corner)
                        assume_inner_corners, assume_outer_corners = assume_outer_corners, assume_inner_corners
                    case 'F':
                        assume_inner_corners.append(add((1, 0), corner))
                        assume_outer_corners.append(add((0, 1), corner))
            case 'F':
                # print('case F')

                match sym:
                    case 'L':
                        assume_inner_corners.append(add((1, 1), corner))
                        assume_outer_corners.append(corner)
                    case '7':
                        assume_inner_corners.append(corner)
                        assume_outer_corners.append(add((1, 1), corner))
                    case 'J':
                        assume_inner_corners.append(add((1, 0), corner))
                        assume_outer_corners.append(add((0, 1), corner))
                        assume_inner_corners, assume_outer_corners = assume_outer_corners, assume_inner_corners
            case '7':
                # print('case F')
                match sym:
                    case 'L':
                        assume_inner_corners.append(add((0, 0), corner))
                        assume_outer_corners.append(add((1, 1), corner))
                        assume_inner_corners, assume_outer_corners = assume_outer_corners, assume_inner_corners
                    case 'F':
                        assume_inner_corners.append(add((1, 0), corner))
                        assume_outer_corners.append(add((0, 1), corner))
                    case 'J':
                        assume_inner_corners.append(add((0, 1), corner))
                        assume_outer_corners.append(add((1, 0), corner))
            case 'J':
                match sym:
                    case 'L':
                        assume_inner_corners.append(add((1, 1), corner))
                        assume_outer_corners.append(corner)
                    case '7':
                        assume_inner_corners.append(corner)
                        assume_outer_corners.append(add((1, 1), corner))
                    case 'F':
                        assume_inner_corners.append(add((0, 1), corner))
                        assume_outer_corners.append(add((1, 0), corner))
                        assume_inner_corners, assume_outer_corners = assume_outer_corners, assume_inner_corners
        prev_sym = sym

    print(max(corner_areas(assume_inner_corners), corner_areas(assume_outer_corners)))

    