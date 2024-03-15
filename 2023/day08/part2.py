from itertools import cycle

def gcd(a, b):
    if a == 1 or b == 0:
        return a
    if a > b:
        return gcd(b, a % b)
    elif a < b:
        return gcd(b, a)
    else:
        return a

with open('input14.txt') as f:
    directions = f.readline().strip()
    f.readline()
    nodes = {}
    for line in f.readlines():
        label, rest = line.strip().split(' = ')
        l, r = rest.strip('()').split(', ')
        nodes[label] = (l, r)
    
    # print(nodes)

    # it happens that
    # 1. There is only one Z node on a path
    # 2. The Z node is always part of the cycle
    # 3. The visit time of the Z node is equal to the length of the cycle
    # So, it suffices to take the LCM of the visit times

    start_nodes = [node for node in nodes if node.endswith('A')]
    visit_times = []
    for snode in start_nodes:
        visited = {}
        current_node = snode
        period = -1
        start = -1
        for i, (di, d) in enumerate(cycle(enumerate(directions))):
            visited[current_node, di] = i
            neighbor_index = 0 if d == 'L' else 1
            prev_node = current_node
            current_node = nodes[current_node][neighbor_index]
            if (current_node, (di + 1) % len(directions)) in visited:
                start = visited[current_node, (di + 1) % len(directions)]
                period = i - start
                break
        for (node, _), visit_time in visited.items():
            if node.endswith('Z'):
                visit_times.append(visit_time)
                break

    lcm_accum = visit_times[0]
    for vt in visit_times[1:]:
        lcm_accum = (lcm_accum * vt) // gcd(vt, lcm_accum)
    
    print(lcm_accum)
    

    


            
                