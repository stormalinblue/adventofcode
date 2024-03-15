import numpy

def step(state, trans):
    return (trans @ state) > 0

def find_start(board):
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == 'S':
                return i, j

def flatten_coord(board, i, j):
    return i * len(board[0]) + j

with open('input21.txt') as f:
    b =  f.read().strip().split()
    start = find_start(b)
    board = [[1 if c == '#' else 0 for c in row.strip()] for row in b]

    board = numpy.array(board, dtype=numpy.int16)

    num_nodes = board.size
    print(num_nodes)

    trans = numpy.zeros((board.size, board.size), dtype=numpy.int16)

    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == 1:
                continue
            # left
            if j - 1 >= 0 and board[i][j - 1] == 0:
                trans[flatten_coord(board, i, j), flatten_coord(board, i, j - 1)] = 1
            # right
            if j + 1 < len(board[0]) and board[i][j + 1] == 0:
                trans[flatten_coord(board, i, j), flatten_coord(board, i, j + 1)] = 1
            # up
            if i - 1 >= 0 and board[i - 1][j] == 0:
                trans[flatten_coord(board, i, j), flatten_coord(board, i - 1, j)] = 1
            # down
            if i + 1 < len(board) and board[i + 1][j] == 0:
                trans[flatten_coord(board, i, j), flatten_coord(board, i + 1, j)] = 1

    state = numpy.zeros((num_nodes), dtype=numpy.int16)
    state[flatten_coord(board, start[0], start[1])] = 1

    for _ in range(64):
        state = step(state, trans)
    
    print(sum(state))

    