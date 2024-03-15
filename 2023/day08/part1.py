from itertools import cycle

with open('input14.txt') as f:
    directions = f.readline().strip()
    f.readline()
    nodes = {}
    for line in f.readlines():
        label, rest = line.strip().split(' = ')
        l, r = rest.strip('()').split(', ')
        nodes[label] = (l, r)
    
    # print(nodes)

    current_node = 'AAA'
    for i, d in enumerate(cycle(directions)):
        neighbor_index = 0 if d == 'L' else 1
        current_node = nodes[current_node][neighbor_index]
        if current_node == 'ZZZ':
            print(i + 1)
            break