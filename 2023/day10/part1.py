import sys
sys.setrecursionlimit(25000)

def ints_in_line(line):
    return list(map(int, line.split()))

def index(board, coord):
    return board[coord[0]][coord[1]]

def directions(letter):
    # print('letter', repr(letter))
    match letter:
        case '7':
            return [(0, -1), (1, 0)]
        case '-':
            return [(0, -1), (0, 1)]
        case 'J':
            return [(-1, 0), (0, -1)]
        case 'S':
            return [(-1, 0), (1, 0), (0, -1), (0, 1)]
        case '|':
            return [(-1, 0), (1, 0)]
        case 'L':
            return [(0, 1), (-1, 0)]
        case 'F':
            return [(0, 1), (1, 0)]

def coord_add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def do_offset(board, coord, letter):
    a = (c for c in (coord_add(coord, d) for d in directions(letter)) if 0 <= c[0] < len(board) and 0 <= c[1] < len(board[0]) and index(board, c) != '.')
    return list(c for c in a if index(board, c) != '.')

def dfs(board, visited, current):
    # print('path', path)
    # print('current', current)
    # neighbors = do_offset(board, current, index(board, current))
    # print('neighbors', neighbors)
    # for n in neighbors:
    #     if n == start:
    #         return path
    #     elif n in visited:
    #         continue
    #     else:
    #         visited.add(current)
    #         path.append(n)
    #         retval = dfs(board, start, path, visited, n)
    #         path.pop()
    #         visited.remove(current)
    #         if retval is not None:
    #             return retval
    # print('could not find')

    neighbors = do_offset(board, current, index(board, current))
    for n in neighbors:
        if n in visited:
            continue
        else:
            visited[n] = current
            dfs(board, visited, n)

def neighbors(board, current):
    return do_offset(board, current, index(board, current))

def dfs_start(board, start):
    parents = {start: None}
    for n in do_offset(board, start, index(board, start)):
        if n not in parents:
            dfs(board, parents, start)
    return parents
    
def leaves(parents):
    candidate_leaf = set(parents)
    for value in parents.values():
        if value is not None:
            candidate_leaf.remove(value)
    return candidate_leaf

with open('input18.txt') as f:
    board = [line for line in f.read().strip().split()]
    print(len(board), len(board[0]))

    start_cell = None
    for i, row in enumerate(board):
        for j, cell in enumerate(row):
            if cell == 'S':
                start_cell = (i, j)
                break
    
    parents = dfs_start(board, start_cell)
    
    loop = list(leaves(parents))
    current = loop[0]
    while current != start_cell:
        current = parents[current]
        loop.append(current)
    
    print(len(loop))
    print(len(loop) // 2)