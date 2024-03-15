import math

import sys
sys.setrecursionlimit(100000)

RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

def mandist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def is_walkable(cell):
    return cell != '#'

def index(board, coord):
    i, j = coord
    return board[i][j]

def set_index(board, coord, value):
    i, j = coord
    board[i][j] = value

def get_adj_list(board):
    adj_list = {}

    for i in range(len(board)):
        for j in range(len(board[0])):
            if is_walkable(board[i][j]):
                al = []
                if i - 1 >= 0 and is_walkable(index(board, (i - 1, j))):
                    al.append((i - 1, j))
                if i + 1 < len(board) and is_walkable(index(board, (i + 1, j))):
                    al.append((i + 1, j))
                if j - 1 >= 0 and is_walkable(index(board, (i, j - 1))):
                    al.append((i, j - 1))
                if j + 1 < len(board[0]) and is_walkable(index(board, (i, j + 1))):
                    al.append((i, j + 1))
                adj_list[(i, j)] = al
    
    return adj_list


def corners(board):
    cn = []
    for i in range(1, len(board) - 1):
        for j in range(1, len(board[0]) - 1):
            wlo = is_walkable(board[i][j - 1]) and not is_walkable(board[i][j + 1])
            wro = is_walkable(board[i][j + 1]) and not is_walkable(board[i][j - 1])
            wdo = is_walkable(board[i + 1][j]) and not is_walkable(board[i - 1][j])
            wuo = is_walkable(board[i - 1][j]) and not is_walkable(board[i + 1][j])
            all_walkable = all(is_walkable(index(board, i)) for i in ((i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)))
            # all_walkable = False
            if is_walkable(board[i][j]) and (wlo or wro or wdo or wuo or all_walkable):
                cn.append((i, j))
    return cn

def has_h_line(board, left, right):
    s = left
    while is_walkable(index(board, s)) and s != right:
        s = add(RIGHT, s)
    return s == right

def has_v_line(board, left, right):
    s = left
    while is_walkable(index(board, s)) and s != right:
        s = add(DOWN, s)
    return s == right

def lines(board, start, end):
    cn = corners(board) + [start, end]
    print('num corners', len(cn))

    cn_by_row = {row: [c for c in cn if c[0] == row] for row in range(len(board))}
    cn_by_col = {col: [c for c in cn if c[1] == col] for col in range(len(board[0]))}

    for row in cn_by_row:
        cn_by_row[row].sort()

    for col in cn_by_col:
        cn_by_col[col].sort()
    
    adj_list = {}

    for row_index, c_list in cn_by_row.items():
        # print(row_index, len(c_list), c_list)
        for i in range(0, len(c_list) - 1):
            s = c_list[i]
            e = c_list[i + 1]

            if has_h_line(board, s, e): 
                if s not in adj_list:
                    adj_list[s] = []
                if e not in adj_list:
                    adj_list[e] = []            
                adj_list[s].append(e)
                adj_list[e].append(s)
    
    for col_index, c_list in cn_by_col.items():
        # print(col_index, len(c_list), c_list)
        for i in range(0, len(c_list) - 1):
            s = c_list[i]
            e = c_list[i + 1]
            

            if has_v_line(board, s, e):
                if s not in adj_list:
                    adj_list[s] = []
                if e not in adj_list:
                    adj_list[e] = []       
                adj_list[s].append(e)
                adj_list[e].append(s)
    
    return adj_list

def compress_adj_list(adj_list):
    cadj = {i: {k: mandist(i, k) for k in j} for i, j in adj_list.items()}

    compressible_nodes = [node for node, edges in cadj.items() if len(edges) == 2]
    print('compressed nodes', len(compressible_nodes))

    for node in compressible_nodes:
        edges = list(cadj[node].keys())
        assert(len(edges) == 2)
        front, back = edges
        cadj[front][back] = cadj[front][node] + cadj[node][back]
        cadj[back][front] = cadj[back][node] + cadj[node][front]
        del cadj[front][node]
        del cadj[back][node]
        del cadj[node]

    return cadj


def paint_path(board, path):
    bc = [list(row) for row in board]
    for i in range(len(path) - 1):
        s, e = path[i], path[i + 1]
        if s[0] > e[0]:
            dir = UP
        elif s[0] < e[0]:
            dir = DOWN
        elif s[1] > e[1]:
            dir = LEFT
        else:
            dir = RIGHT

        w = s
        while w != e:
            set_index(bc, w, 'O')
            w = add(dir, w)
        set_index(bc, e, 'O')

    return '\n'.join(''.join(row) for row in bc)



def longest_path(adj_list, start, end):
    def longest_path_helper(visited, current):
        # print('current', current)
        if current == end:
            # print('found!')
            return 0, [end]
        # print('all neighbors', adj_list[current])
        # print('all visited', visited)
        max_length = 0
        best_path_after = None
        found = False
        for n in adj_list[current]:
            # print('n is', n)
            # differs_in_one_dim(n, current)
            if n not in visited:
                # print('visiting neighbor', n)
                visited.add(n)
                plen, path = longest_path_helper(visited, n)
                if plen is not None:
                    # print('found pred', current, 'with length', length)
                    if plen + adj_list[current][n] > max_length:
                        best_path_after = path
                        max_length = plen + adj_list[current][n]
                    found = True
                visited.remove(n)
        return (max_length, None) if found else (None, None)
    return longest_path_helper(set([start]), start)


def differs_in_one_dim(a, b):
    assert(a[0] == b[0] or a[1] == b[1])

def print_path(path):
    prev = path[0]
    for point in path[1:]:
        d = mandist(point, prev)
        differs_in_one_dim(point, prev)
        if point[0] > prev[0]:
            print('d', d, point)
        elif point[0] < prev[0]:
            print('u', d, point)
        elif point[1] > prev[1]:
            print('r', d, point)
        elif point[1] < prev[1]:
            print('l', d, point)
        prev = point

with open('input23.txt') as f:
    board = f.read().strip().split()

    rows, cols = len(board), len(board[0])
    print(rows, cols)

    start = (0, 1)
    end = (rows - 1, cols - 2)

    # print(lines(board, start, end))
    alist = lines(board, start, end)
    result = compress_adj_list(alist)
    print('nodes after compression', len(result))
    print(longest_path(result, start, end))
    # plen, pth = longest_path(alist, start, end)
    # print_path(pth)
    # pth_len = 0
    # for i in range(len(pth) - 1):
    #     pth_len += mandist(pth[i], pth[i + 1])
    #     differs_in_one_dim(pth[i], pth[i + 1])
    # # print_path(pth)
    # new_pth = [pth[0]]
    # new_pth_len = 0
    # prev_pth_len = 0
    # prev_paint_len = 0
    # new_paint = None
    # prev_paint = None
    # for i in range(1, len(pth)):
    #     new_pth_len += mandist(pth[i - 1], pth[i])
    #     new_pth.append(pth[i])

    #     new_paint = paint_path(board, new_pth)

    #     paint_len = new_paint.count('O') - 1
    #     if paint_len != new_pth_len:
    #         print('mismatch at', i, pth[i], paint_len, new_pth_len, mandist(pth[i - 1], pth[i]), 'prev', prev_paint_len, prev_pth_len)
            
    #         print('offending paint')
    #         print(new_paint)
    #         print('prev_paint')
    #         print(prev_paint)
    #         break
    #     prev_pth_len = new_pth_len
    #     prev_paint_len = paint_len
    #     prev_paint = new_paint
    # print(plen)

    # print(paint_path(board, pth))
    