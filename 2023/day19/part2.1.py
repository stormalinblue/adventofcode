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
    
def complement_single_bound(a):
    match a:
        case (0, 4000):
            return []
        case (0, hi):
            return [(hi + 1, 4000)]
        case (low, 4000):
            return [(0, low - 1)]
        case (a, b):
            return [(0, a - 1), (b + 1, 4000)]


def complement_bounds(bounds):
    result = [(0, 4000)]
    for bound in bounds:
        result = and_bounds(result, complement_single_bound(bound))
    return result

def and_single_bound(a, b):
    lo = max(a[0], b[0])
    hi =  min(a[1], b[1])
    if lo <= hi:
        return [(lo, hi)]
    else:
        return []

def intersects(a, b):
    if ((a[0] <= b[0] <= b[1] <= a[1]) or
    (b[0] <= a[0] <= b[1] <= a[1]) or
    (a[0] <= b[0] <= a[1] <= b[1]) or
    (b[0] <= a[0] <= a[1] <= b[1])):
        return True
    else:
        return False

def or_single_bound(a, b):
    if intersects(a, b):
        return [(min(a[0], b[0]), max(a[1], b[1]))]
    else:
        return sorted([a, b])

def and_bounds(a, b):
    res = []
    for bound_a in a:
        for bound_b in b:
            res += and_single_bound(bound_a, bound_b)
    return res

def or_bounds(a, b):
    l = sorted(a + b)
    res = l
    changed = True
    while changed:
        changed = False
        new_res = []
        for i in range(i + 1)



def complement_wcases(wcases):
    return {i: complement_bounds(b) for i, b in wcases.items()}

def and_wcases(wcases_a, wcases_b):
    return {i: and_bounds(wcases_a[i], wcases_b[i]) for i in wcases_a}

def or_wcases(wcases_a, wcases_b):
    return {i: or_bounds(wcases_a[i], wcases_b[i]) for i in wcases_a}

def process_wcases(cases):
    for case in cases:
        bounds = {
            'x': [(1, 4000)],
            'm': [(1, 4000)],
            'a': [(1, 4000)],
            's': [(1, 4000)]
        }

        if len(case) == 4:
            if case[0] == '<':
                l = case[1]
                bounds[l] = (bounds[l][0], case[2] - 1)
            elif case[1] == '>':


with open('input19.txt') as f:
    workflow_str, parts_str = f.read().strip().split('\n\n')
    workflow_lines = workflow_str.split('\n')
    parsed_workflows = [parse_workflow(line) for line in workflow_lines]
    # print(parsed_workflows)
    workflows = {name: cases for name, cases in parsed_workflows}
    # print(workflows)
    parts = [parse_part(line) for line in parts_str.split()]
    # print(parts)

