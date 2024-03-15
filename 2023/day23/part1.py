import math

import sys
sys.setrecursionlimit(100000)

RIGHT = (0, 1)
DOWN = (1, 0)
UP = (-1, 0)
LEFT = (0, -1)

def add(coord1, coord2):
    return (coord1[0] + coord2[0], coord1[1] + coord2[1])

def is_walkable(cell):
    return cell != '#'

def index(board, coord):
    i, j = coord
    return board[i][j]

def get_adj_list(board):
    adj_list = {}

    for i in range(len(board)):
        for j in range(len(board[0])):
            if is_walkable(board[i][j]):
                al = []
                if board[i][j] in '.^' and i - 1 >= 0 and is_walkable(index(board, (i - 1, j))):
                    al.append((i - 1, j))
                if board[i][j] in '.v' and i + 1 < len(board) and is_walkable(index(board, (i + 1, j))):
                    al.append((i + 1, j))
                if board[i][j] in '.<' and j - 1 >= 0 and is_walkable(index(board, (i, j - 1))):
                    al.append((i, j - 1))
                if board[i][j] in '.>' and j + 1 < len(board[0]) and is_walkable(index(board, (i, j + 1))):
                    al.append((i, j + 1))
                adj_list[(i, j)] = al
    
    return adj_list

# def bellman_ford(adj_list, start):
#     distance = {}
#     predecessor = {}
#     for node in adj_list:
#         distance[node] = math.inf
#         predecessor[node] = None
    
#     distance[start] = 0

#     for _ in range(len(adj_list) - 1):
#         for u, edges in adj_list.items():
#             for v in edges:
#                 if distance[u] - 1 < distance[v]:
#                     distance[v] = distance[u] - 1
#                     predecessor[v] = u
    
#     for u, edges in adj_list.items():
#         for v in edges:
#             if distance[u] - 1 < distance[v]:
#                 predecessor[v] = u
#                 visited = set()
#                 while u not in visited:
#                     visited.add(u)
#                     u = predecessor[u]
#                 nyncle = 
#     return distance, predecessor

def longest_path(adj_list, start, end):
    def longest_path_helper(visited, current):
        # print('current', current)
        if current == end:
            # print('found!')
            return 0
        # print('all neighbors', adj_list[current])
        # print('all visited', visited)
        max_length = 0
        found = False
        for n in adj_list[current]:
            # print('n is', n)
            if n not in visited:
                # print('visiting neighbor', n)
                visited.add(n)
                length = longest_path_helper(visited, n)
                if length is not None:
                    # print('found pred', current, 'with length', length)
                    max_length = max(length, max_length)
                    found = True
                visited.remove(n)
        return (max_length + 1) if found else None
    return longest_path_helper(set([start]), start)


with open('input23.txt') as f:
    board = f.read().strip().split()

    rows, cols = len(board), len(board[0])

    start = (0, 1)
    end = (rows - 1, cols - 2)
    al = get_adj_list(board)
    d = longest_path(al, start, end)
    print(d)

    