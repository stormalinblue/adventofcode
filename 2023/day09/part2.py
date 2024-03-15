def consec_diff(num_line):
    result = []
    for i in range(len(num_line) - 1):
        result.append(num_line[i + 1] - num_line[i])
    return result

def all_zeros(num_line):
    return all(i == 0 for i in num_line)

with open('input16.txt') as f:
    number_lines = [list(map(int, line.strip().split())) for line in f.readlines()]
    
    s = 0
    for num_line in number_lines:
        # print('number line')
        tnumline = list(num_line)
        tnumlines = [tnumline]
        # print(tnumline)
        while not all_zeros(tnumline):
            tnumline = consec_diff(tnumline)
            # print(tnumline)
            tnumlines.append(tnumline)
        
        srev = list(reversed(tnumlines))
        last_extra = 0
        for tnumline in srev[1:]:
            last_extra = tnumline[0] - last_extra
        s += last_extra
    print(s)

        
