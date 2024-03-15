RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

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

def connected(a, b):
    return abs(sum(add(a, nega(b)))) == 1

def connected_ranges(walls):
    print(walls)
    if not walls:
        return []
    
    walls = walls + [(-1, -1)]
    
    ranges = []
    curr_start = walls[0][1]
    index = 1
    while index < len(walls):
        while index < len(walls) and connected(walls[index - 1], walls[index]):
            index += 1
        ranges.append((curr_start, walls[index - 1][1]))
        if index < len(walls):
            # print('done connected', index, walls[index])
            curr_start = walls[index][1]
            index += 1
    # ranges.append((curr_start, walls[len(walls) - 1][1]))
    return ranges

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

with open('input18.txt') as f:
    moves = []
    for line in (l.strip() for l in f.read().strip().split('\n')):
        # print(line)
        d, l, c = line.split()

        dir = d
        l = int(l)
        col = c[2:-1]
        moves.append((dir, l, col))
    
    move_syms = []
    prev_dir = moves[-1][0]
    for (d, _, _) in moves:
        if prev_dir == d:
            match d:
                case 'L':
                    sym = '-'
                case 'R':
                    sym = '-'
                case 'D':
                    sym = '|'
                case 'U':
                    sym = '|'
        else:
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
    for (d, l, _), sym in zip(moves, move_syms):
        match d:
            case 'R':
                dir = RIGHT
            case 'L':
                dir = LEFT
            case 'U':
                dir = UP
            case 'D':
                dir = DOWN
        positions.append((cur_pos, sym))
        cur_pos = add(cur_pos, dir)
        for i in range(2, l + 1):
            positions.append((cur_pos, '-' if d in 'RL' else '|'))
            cur_pos = add(cur_pos, dir)
    
    # print(positions)
    

    min_row = min(i for ((i, _), _) in positions)
    min_col = min(i for ((_, i), _) in positions)
    max_row = max(i for ((i, _), _) in positions)
    max_col = max(i for ((_, i), _) in positions)
    rows = max_row - min_row + 1
    cols = max_col - min_col + 1

    print("dimensions", (max_row - min_row + 1), (max_col - min_col + 1))

    offset = nega((min_row, min_col))
    print('offset', offset)
    o_positions = [(add(offset, pos), sym) for (pos, sym) in positions]

    # print(o_positions)

    walls = {}
    for pos, dir in o_positions:
        row = pos[0]
        if row not in walls:
            walls[row] = []
        
        walls[row].append((pos, dir))
    
    # print('walls', walls)
    
    for row in range(rows):
        walls[row] = list(set(walls[row]))
        walls[row].sort()

    s = 0
    for row in range(rows):
        s += wall_area(walls[row])
        # print('s is now', s)
    print(s)
    # print(connected((0, 1), (0, 3)))
    # print(connected_ranges([(0, 0), (0, 1), (0, 3), (0, 4), (0, 6), (0, 8)]))
    
    print(paint(rows, cols, o_positions))
    
    # print(walls)