def print_board(board):
    return '\n'.join(''.join(row) for row in board)

def fall_up(board):
    for column in range(len(board[0])):
        for orig_row in range(len(board)):
            if board[orig_row][column] == 'O':
                # print('found', orig_row, column)
                board[orig_row][column] = '.'
                row = orig_row
                while row - 1 >= 0 and board[row - 1][column] not in '#O':
                    row -= 1
                board[row][column] = 'O'

def rotate_cw(board):
    return [[board[len(board) - 1 - i][j] for i in range(len(board))] for j in range(len(board[0]))]

def copy_board(board):
    return [list(row) for row in board]

def weigh(board):
    s = 0
    for row, line in enumerate(board):
        s += sum((len(board) - row) for c in line if c == 'O')        
    return s

with open('input.txt') as f:
    board = [list(row) for row in f.read().strip().split()]

    print(print_board(board))

    hashes = {}
    rev_hash = {}

    hash_found = None
    hash_at = None

    for i in range(1000000000):
        if i % (1000000000 // 100) == 0:
            print(i)
        for _ in range(4):
            fall_up(board)
            board = rotate_cw(board)
        hash = print_board(board)
        if hash in hashes:
            print('hash found', i)
            hash_found = i
            print('hash at', hashes[hash])
            hash_at = hashes[hash]
            break
        else:
            hashes[hash] = i
            rev_hash[i] = hash

    b = 1000000000
    mod_offset = (b - 1 - hash_at) % (hash_found - hash_at)
    print('mod_offset', mod_offset)
    # for i in range(9):
    #     print(rev_hash[i])
    #     print('=' * 8)
    # fall_up(board)
    # print('=' * 10)
    # print(print_board(board))
    print(weigh([list(row) for row in rev_hash[hash_at + mod_offset].split()]))