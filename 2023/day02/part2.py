def col_from_string(line):
    pair_strs = line.strip().split(',')
    col = {}
    for pair_str in pair_strs:
        # print(pair_str)
        n, c = pair_str.split()
        # print(n, c)
        col[c] = int(n)
    return col

def draws_from_string(line):
    g, r = line.split(':')
    _, n = g.split()
    n = int(n)
    
    test_lines = r.strip().split(';')
    # print('tl', test_lines)
    return n, [col_from_string(tl) for tl in test_lines]

colors = set(['red', 'green', 'blue'])
req = {'red': 12, 'blue': 14, 'green': 13}

with open('input3.txt') as f:
    s = 0
    for line in f:
        test_num, draws = draws_from_string(line)
        failed = False
        mins = {c: 0 for c in colors}
        for color in colors:
            for draw in draws:
                if color in draw and draw[color] > mins[color]:
                    mins[color] = draw[color]
        power = mins['red'] * mins['blue'] * mins['green']
        s += power
    print(s)