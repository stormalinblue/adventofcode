from itertools import combinations
from random import sample
from collections import deque
from math import inf

def connected_components(adj_list):
    nodes = set(adj_list)
    components = []
    while nodes:
        start = nodes.pop()
        s = [start]
        visited = set([start])
        c = set([start])
        while s:
            # print(s)
            v = s.pop()
            for neighbor in adj_list[v]:
                if neighbor not in visited:
                    nodes.remove(neighbor)
                    c.add(neighbor)
                    visited.add(neighbor)
                    s.append(neighbor)
                    
        components.append(c)
    # print(components)
    return components

def bfs(flow_graph, flow_caps, source, sink):
    visited = set()
    queue = deque()
    queue.append(source)
    visited.add(source)
    parent = {source: None}

    # print('adj_list before', adj_list)

    while queue:
        u = queue.popleft()

        for neighbor in flow_graph[u]:
            if neighbor not in visited and flow_graph[u][neighbor] < flow_caps[u][neighbor]:
                queue.append(neighbor)
                visited.add(neighbor)
                parent[neighbor] = u
    
    return sink in visited, parent

def ford_fulkerson(adj_list, source, sink):
    flow_graph = {v: {n: 0 for n in adj_list[v]} for v in adj_list}
    flow_caps = {v: {n: 1 for n in adj_list[v]} for v in adj_list}

    max_flow = 0

    while True:
        found, parent = bfs(flow_graph, flow_caps, source, sink)
        if not found:
            break
        path_flow = inf
        s = sink
        while s != source:
            p = parent[s]
            path_flow = min(path_flow, flow_caps[p][s] - flow_graph[p][s])
            s = p
        
        max_flow += path_flow

        v = sink
        while v != source:
            u = parent[v]
            flow_graph[u][v] += path_flow
            flow_graph[v][u] -= path_flow
            v = u
        
    zero_count = 0
    e = set()
    for u in flow_graph:
        for v in flow_graph[u]:
            if flow_graph[u][v] == flow_caps[u][v]:
                zero_count += 1
                e.add(canonical_edge(u, v))
    
    # pprint(flow_graph)

    return max_flow, sorted(e)

def canonical_edge(a, b):
    if a < b:
        return (a, b)
    else:
        return (b, a)
    
def disconnect_edges(adj_list, edges):
    for edge in edges:
        i, j = edge
        adj_list[i].remove(j)
        adj_list[j].remove(i)

def connect_edges(adj_list, edges):
    for edge in edges:
        i, j = edge
        adj_list[i].add(j)
        adj_list[j].add(i)

def prod(l):
    p = 1
    for i in l:
        p *= i
    return p

from pprint import pprint
r = 3
with open('input25.txt') as f:
    connections = {}
    edges = []
    
    for line in f:
        i, o = line.strip().split(': ')
        if i not in connections:
            connections[i] = set()
        os = o.strip().split()
        for a in os:
            if a not in connections:
                connections[a] = set()
            connections[a].add(i)
            connections[i].add(a)
            edges.append(canonical_edge(a, i))
    
    print('hello')
    # pprint(connections)

    print(len(edges))

    print(min([len(e) for n, e in connections.items()]))
    # print(connections['hdz'])

    nodes = list(connections)

    found = False
    for i in range(len(nodes)):
        for j in range(i + 1, len(nodes)):
            if i == j:
                continue
            min_cut, candidate_edges = ford_fulkerson(connections, nodes[i], nodes[j])
            if min_cut == r:
                for combo in combinations(candidate_edges, r):
                    disconnect_edges(connections, combo)
                    comp = connected_components(connections)
                    if len(comp) == 2:
                        print([len(c) for c in comp])
                        print(prod([len(c) for c in comp]))
                        found = True
                        break
                    connect_edges(connections, combo)
                if found:
                    break
        if found:
            break

            
    
    # i = 0
    # tot = 6401532375
    # while True:
    #     combo = sample(edges, 3)
    #     # print('combo', list(sorted(combo)))
    #     if i % (10 ** 3) == 0:
    #         print(i, f'{i * 100/tot}%')
    #     disconnect_edges(connections, combo)
    #     comp = connected_components(connections)
    #     if len(comp) == 2:
    #         print([len(c) for c in comp])
    #         print(prod([len(c) for c in comp]))
    #         break
    #     connect_edges(connections, combo)
    #     i += 1

