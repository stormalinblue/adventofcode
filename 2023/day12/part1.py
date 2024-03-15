import sys, math

def contiguous(string):
    # print('con', string)
    if not string:
        return []
    if string[0] != '#':
        return contiguous(string[1:])
    i = 0
    while i < len(string) and string[i] == '#':
        i += 1
    return [i] + contiguous(string[i:])

def satisfies(string, constraints):
    return contiguous(string) == constraints

def strings_builder(string):
    if not string:
        return ['.']
    if string[0] == '.':
        return ['.' + s for s in strings_builder(string[1:])]
    elif string[0] == '#':
        return ['#' + s for s in strings_builder(string[1:])]
    else:
        ls = strings_builder(string[1:])
        return ['.' + s for s in ls] + ['#' + s for s in ls]
    

with open('input12.txt') as f:
    problems = []
    for line in f.read().strip().split('\n'):
        # print(line.count('#'))
        prob, _sat = line.split()
        # print(_sat, _sat.split(','))
        sat = [int(i) for i in _sat.split(',')]
        problems.append((prob, sat))
    # print(problems)

    ans = 0
    for prob, sat in problems:
        # print('prob', prob)
        for s in strings_builder(prob):
            # print(s)
            if satisfies(s, sat):
                ans += 1
                # print('satisfied')
    print(ans)
    

