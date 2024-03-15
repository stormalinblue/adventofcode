from collections import deque

def index(box, coord):
    return box[coord[0]][coord[1]]

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

def num_energized(box, start):
    frontier = deque([start])

    energized = set()
    while frontier:
        t = frontier.popleft()
        
        t_coord, dir = t

        energized.add(t)
        match index(box, t_coord):
            case '.':
                next_dir = [dir]
            case '\\':
                if dir == DOWN:
                    next_dir = [RIGHT]
                elif dir == UP:
                    next_dir = [LEFT]
                elif dir == RIGHT:
                    next_dir = [DOWN]
                elif dir == LEFT:
                    next_dir = [UP]
            case '/':
                if dir == DOWN:
                    next_dir = [LEFT]
                elif dir == UP:
                    next_dir = [RIGHT]
                elif dir == RIGHT:
                    next_dir = [UP]
                elif dir == LEFT:
                    next_dir = [DOWN]
            case '|':
                if dir == LEFT or dir == RIGHT:
                    next_dir = [UP, DOWN]
                else:
                    next_dir = [dir]
            case '-':
                if dir == UP or dir == DOWN:
                    next_dir = [LEFT, RIGHT]
                else:
                    next_dir = [dir]
        
        for d in next_dir:
            n_c = add(t_coord, d)
            # print('considering next', n_c)
            if not (0 <= n_c[0] < len(box) and 0 <= n_c[1] < len(box[0])):
                continue
            if (n_c, d) not in energized:
                frontier.append((n_c, d))

    return (len(set(c for c, d in energized)))

with open('test.txt') as f:
    box = f.read().strip().split()

    down_max = max(num_energized(box, ((0, i), DOWN)) for i in range(len(box[0])))
    up_max = max(num_energized(box, ((len(box) - 1, i), UP)) for i in range(len(box[0])))
    right_max = max(num_energized(box, ((i, 0), RIGHT)) for i in range(len(box)))
    left_max = max(num_energized(box, ((i, len(box[0]) - 1), LEFT)) for i in range(len(box)))
    print(max([down_max, up_max, right_max, left_max]))
        