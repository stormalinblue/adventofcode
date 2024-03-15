def numbers_in_line(line, index):
    if not line:
        return []
    number_str = ""
    for char in line:
        if not char.isdigit():
            break
        number_str += char
    if number_str:
        num_len = len(number_str)
        return [(int(number_str), (index, index + num_len - 1))] + numbers_in_line(line[num_len:], index + num_len)
    else:
        return numbers_in_line(line[1:], index + 1)

with open('input5.txt') as f:
    lines = f.read().strip().split('\n')
    # print(lines)

    s = 0

    numbers_in_lines = [numbers_in_line(line, 0) for line in lines]
    for index in range(len(lines)):
        for (number, (start, end)) in numbers_in_lines[index]:
            ranges = []

            if index > 0:
                ranges.append(lines[index - 1][max(start - 1, 0):end + 2])
            if index < len(lines) - 1:
                ranges.append(lines[index + 1][max(start - 1, 0):end + 2])
            ranges.append(
                lines[index][max(start - 1, 0):end + 2]
            )

            is_part_number = any(any(not (c.isdigit() or c == '.') for c in r) for r in ranges)
            if is_part_number:
                s += number
    
    print(s)

