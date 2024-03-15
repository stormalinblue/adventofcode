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
        for draw in draws:
            if not (all(key in colors for key in draw)):
                failed = True
                break
            elif not all(v <= req[k] for k, v in draw.items()):
                failed = True
                break
        if not failed:
            print('test', test_num, 'passed')
            s += test_num
    print(s)