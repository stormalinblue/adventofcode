from itertools import product

def parse_workflow(workflow_line):
    workflow_name, _rest = workflow_line.split('{')
    cases = _rest[:-1].split(',')
    parsed_cases = []
    for case in cases:
        if ':' in case:
            cmp, target = case.split(':')
            op = None
            if '<' in case:
                left, right = cmp.split('<')
                op = '<'
            elif '>' in case:
                left, right = cmp.split('>')
                op = '>'
            parsed_cases.append((op, left, int(right), target))
        else:
            parsed_cases.append((case,))
    return workflow_name, parsed_cases

def parse_part(part_line):
    return {i: int(n) for i, n in (v.split('=') for v in part_line[1:-1].split(','))}

def accepts(part, workflows, patience=10000):
    w = 'in'
    _pat = patience
    visited = set()
    while w not in ['A', 'R']:
        visited.add(w)
        for case in workflows[w]:
            if len(case) == 4:
                if case[0] == '<':
                    if part[case[1]] < case[2]:
                        w = case[3]
                        break
                elif case[0] == '>':
                    if part[case[1]] > case[2]:
                        w = case[3]
                        break
            else:
                w = case[0]
                break
        if w in visited:
            break
        _pat -= 1
        if _pat < 0:
            break
    return w == 'A'

def process_sections(s):
    q = sorted(s, key=lambda x: x[1])
    l = []
    for i in q:
        if not l:
            l.append(i)
        elif l[-1] != i:
            l.append(i)
    return l

def prod(l):
    p = 1
    for i in l:
        p *= i
    return p

def num_poss(xmas_list, workflows):
    part = {i: j for i, j in zip('xmas', (l[0] for l in xmas_list))}
    if accepts(part, workflows):
        return prod(l[1] - l[0] + 1 for l in xmas_list)
    else:
        return 0

with open('input19.txt') as f:
    workflow_str, parts_str = f.read().strip().split('\n\n')
    workflow_lines = workflow_str.split('\n')
    parsed_workflows = [parse_workflow(line) for line in workflow_lines]
    # print(parsed_workflows)
    workflows = {name: cases for name, cases in parsed_workflows}
    # print(workflows)
    parts = [parse_part(line) for line in parts_str.split()]
    # print(parts)

    sections = {i: [] for i in 'xmas'}

    for w in workflows:
        for case in workflows[w]:
            if len(case) == 4:
                sections[case[1]].append((case[0], case[2]))
    
    for s in sections:
        sections[s] = process_sections(sections[s])

    print(sections)

    new_sections = {i: [] for i in 'xmas'}
    for i in 'xmas':
        s = sections[i]
        first = sections[i][0]
        last = sections[i][-1]
        first_end = (first[1] - 1) if first[0] == '<' else first[1]
        last_start = (last[1]) if last[0] == '<' else (last[1] + 1)
        new_sections[i].append((1, first_end))
        assert(1 < s[0][1])
        for k in range(len(s) - 1):
            curr = sections[i][k]
            next = sections[i][k + 1]
            start = curr[1] if curr[0] == '<' else (curr[1] + 1)
            end = (next[1] - 1) if next[0] == '<' else next[1]
            new_sections[i].append((start, end))
        assert(sections[i][-1][1] < 4000)
        new_sections[i].append((last_start, 4000))

    print(new_sections)
    
    tot = prod(len(k) for k in new_sections.values())
    print('tot', tot)

    s = 0
    c = 0
    k = 0
    for xmas_list in product(new_sections['x'], new_sections['m'], new_sections['a'], new_sections['s']):
        # if c * 100 >= tot * k:
        #     print(k, 'percent')
        #     k += 1
        if c % 1000000 == 0:
            print(f'{c}, {(c/tot) * 100:2f}%')
        c += 1
        s += num_poss(xmas_list, workflows)
    print(s)



