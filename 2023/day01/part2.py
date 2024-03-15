digit_map = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
}

def digits(line):
    if not line:
        return []
    if line[0].isdigit():
        return [int(line[0])] + digits(line[1:])
    else:
        for digit_name in digit_map:
            if line.startswith(digit_name):
                return [digit_map[digit_name]] + digits(line[1:])
        return digits(line[1:])

with open('input2') as f:
    s = 0
    for line in f:
        d = digits(line.strip())
        print(line.strip(), d)
        num = 10 * d[0] + d[-1]
        if num >= 100:
            raise ValueError
        print(num, s)
        s += num
    print(s)