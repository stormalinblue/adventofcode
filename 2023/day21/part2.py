import numpy
from itertools import product


RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

def find_start(board):
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == 'S':
                return i, j


def normalize_coord(board, coord):
    i, j = coord
    return ((i + len(board)) % len(board), (j + len(board[0])) % len(board[0]))


def transition_map(board):
    trans = {}
    def trans_add(coord, target):
        if coord not in trans:
            trans[coord] = []
        trans[coord].append(target)
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == 1:
                continue
            # left
            if board[normalize_coord(board, (i, j - 1))] == 0:
                trans_add((i, j), LEFT)
            # right
            if board[normalize_coord(board, (i, j + 1))] == 0:
                trans_add((i, j), RIGHT)
            # up
            if board[normalize_coord(board, (i - 1, j))] == 0:
                trans_add((i, j), UP)
            # down
            if board[normalize_coord(board, (i + 1, j))] == 0:
                trans_add((i, j), DOWN)
    return trans

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

def flatten_coord(board, i, j):
    return i * len(board[0]) + j



def sim_steps(board, sim_set, start, tmap, num_steps):
    # print('sim_steps', num_steps)
    if num_steps == 0:
        return {i: set([(0, 0)]) for i in sim_set}

    else:
        half_steps = num_steps // 2
        print(num_steps, 'half sim')
        half_sim_steps = sim_steps(board, sim_set, start, tmap, half_steps)
        # print(num_steps, 'half sim steps', len(half_sim_steps), (5, 1) in half_sim_steps, half_sim_steps[5, 1])
        result = {}
        for src_coord in half_sim_steps:
            result[src_coord] = set()
            for half_dst_coord in half_sim_steps[src_coord]:
                n_hdc = normalize_coord(board, add(src_coord, half_dst_coord))
                if n_hdc in half_sim_steps:
                    for dst_coord in half_sim_steps[n_hdc]:
                        if src_coord == start:
                            print('adding', half_dst_coord, 'and', dst_coord)
                        result[src_coord].add(add(half_dst_coord, dst_coord))
        print(num_steps, 'half sim done')
        
        if num_steps % 2 == 1:
            print(num_steps, 'simulating single step')
            new_result = {}
            for src_coord in result:
                if src_coord not in new_result:
                    new_result[src_coord] = set()
                if src_coord == start:
                    print('src', src_coord, tmap[src_coord])
                for src_disp in result[src_coord]:
                    n_hdc = normalize_coord(board, add(src_coord, src_disp))
                    if n_hdc in tmap:
                        for disp in tmap[n_hdc]:
                            if src_coord == start:
                                print('adding', src_disp, 'and', disp)
                            new_result[src_coord].add(add(src_disp, disp))
            print(num_steps, 'simulating single step done')
            return new_result
        else:
            return result

        return result
        

    

with open('test.txt') as f:
    b =  f.read().strip().split()
    start = find_start(b)
    board = [[1 if c == '#' else 0 for c in row.strip()] for row in b]

    board = numpy.array(board, dtype=numpy.int16)

    num_nodes = board.size
    print(num_nodes, board.shape)

    trans = transition_map(board)

    # print(trans)

    # sim_set = set([(i, j) for i, j in product(range(len(board)), range(len(board[0]))) if board[i][j] == 0])
    sim_set = set([start])
    # print(len(sim_set))

    poss = sim_steps(board,  sim_set, start, trans, 2)
    
    print(len(poss[start]))