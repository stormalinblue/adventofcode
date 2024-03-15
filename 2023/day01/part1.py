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
                return [digit_map] + digits[len(digit_name):]
        return digits(line[1:])

with open('input.txt') as f:
    s = 0
    for line in f:
        digits = [int(c) for c in line if c.isdigit()]
        # print(line, digits)
        s += 10 * digits[0] + digits[-1]
    print(s)