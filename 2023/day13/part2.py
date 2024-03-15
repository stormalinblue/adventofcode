def palindromes(strings):
    cols = []
    for w in range(2, len(strings), 2):
        if strings[:w] == list(reversed(strings[:w])):
            cols.append(w // 2)
    for w in range(2, len(strings), 2):
        if strings[len(strings) - w:] == list(reversed(strings[len(strings) - w:])):
            cols.append(len(strings) - (w // 2))
    return cols

def transpose(strings):
    new_strings = []
    for i in range(len(strings[0])):
        new_strings.append([string[i] for string in strings])
    return new_strings

def changes(block):
    block_t = transpose(block)
    orig_cols = palindromes(block)
    orig_rows = palindromes(block_t)

    for i in range(len(block)):
        for j in range(len(block[0])):
            orig_cell = block[i][j]
            block[i][j] = '.' if block[i][j] == '#' else '#'
            new_cols = palindromes(block)
            block[i][j] = orig_cell
            block_t[j][i] = '.' if block[i][j] == '#' else '#'
            new_rows = palindromes(block_t)
            block_t[j][i] = orig_cell

            if not (new_cols == orig_cols and new_rows == orig_rows) and (new_cols or new_rows):
                res = sum(set(new_rows) - set(orig_rows)) + 100 * sum(set(new_cols) - set(orig_cols))
                print(i, j, orig_rows, new_rows, orig_cols, new_cols, res)
                return res
    return 0

with open('input13.txt') as f:
    _blocks = f.read().strip().split('\n\n')
    print(len(_blocks))

    blocks = [[list(row) for row in block.split()] for block in _blocks]
    # print(blocks)
    print(sum([changes(block) for block in blocks]))
    # print(rows)
# 
    # print(sum(sum(row) for row in rows) + 100 * sum(sum(col) for col in cols))