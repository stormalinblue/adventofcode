def hsh(s):
    h = 0
    for c in s:
        h += ord(c)
        h *= 17
        h %= 256
    return h

with open('input15.txt') as f:

    steps = []
    s = 0
    for step in f.read().strip().split(','):
        if step.endswith('-'):
            label = step[:-1]
            steps.append(('-', label, hsh(label)))
        else:
            label, _l = step.split('=')
            l = int(_l)
            steps.append(('+', label, hsh(label), l))
    

    boxes = {}
    step_id = 0
    for step in steps:
        step_id += 1
        if step[0] == '-':
            if step[2] in boxes and step[1] in boxes[step[2]]:
                del boxes[step[2]][step[1]]
        else:
            if step[2] not in boxes:
                boxes[step[2]] = {}
            
            if step[1] not in boxes[step[2]]:
                boxes[step[2]][step[1]] = (step_id, step[3])
            else:
                boxes[step[2]][step[1]] = (boxes[step[2]][step[1]][0], step[3])
    
    s = 0
    for box, row in boxes.items():
        if not row:
            continue
        local_s = 0
        for i, (_, f) in enumerate(sorted(row.values())):
            d = (i + 1) * (box + 1) * f
            print('d', d)
            local_s += d
        print('local', local_s)
        s += local_s
    print(s)
    print(boxes)