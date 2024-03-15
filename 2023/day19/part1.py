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

with open('input19.txt') as f:
    workflow_str, parts_str = f.read().strip().split('\n\n')
    workflow_lines = workflow_str.split('\n')
    parsed_workflows = [parse_workflow(line) for line in workflow_lines]
    # print(parsed_workflows)
    workflows = {name: cases for name, cases in parsed_workflows}
    # print(workflows)
    parts = [parse_part(line) for line in parts_str.split()]
    # print(parts)

    s = 0
    for part in parts:
        w = 'in'
        while w not in ['A', 'R']:
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
        if w == 'A':
            s += sum(part.values())

    print(s)


