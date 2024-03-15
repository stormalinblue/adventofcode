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

def print_matrix(matrix):
    print('\n'.join(''.join(chr(ord('A') - 1 + e) if e != 0 else '.' for e in row) for row in matrix))

with open('input6.txt') as f:
    lines = f.read().strip().split('\n')
    number_names = [[0 for _ in range(len(lines[0]))] for _ in range(len(lines))]

    number_index = 1
    number_map = {}

    numbers_in_lines = [numbers_in_line(line, 0) for line in lines]
    for index in range(len(lines)):
        line_numbers = numbers_in_lines[index]
        for (number, (start, end)) in line_numbers:
            number_map[number_index] = number
            for lr_index in range(start, end + 1):
                number_names[index][lr_index] = number_index
            number_index += 1
    
    s = 0

    for row_index in range(len(lines)):
        for col_index in range(len(lines[0])):
            if lines[row_index][col_index] == '*':
                # print('candidate', row_index, col_index)
                num_indices = []

                if row_index > 0:
                    above = number_names[row_index - 1][max(col_index - 1, 0):col_index + 2]
                    num_indices.extend(above)
                    # print('above', above)
                if row_index < len(lines) - 1:
                    below = number_names[row_index + 1][max(col_index - 1, 0):col_index + 2]
                    num_indices.extend(below)
                    # print('below', below)
                num_indices.extend(number_names[row_index + 0][max(col_index - 1, 0):col_index + 2])

                # print(num_indices)

                num_indices = set(num_indices)
                num_indices.remove(0)
                if len(num_indices) == 2:
                    # print(row_index, col_index, num_indices)
                    p = 1
                    for name in num_indices:
                        p *= number_map[name]
                    # print(p)
                    s += p
    
    
    print(s)

