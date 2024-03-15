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
        new_strings.append(''.join(string[i] for string in strings))
    return new_strings

with open('input13.txt') as f:
    _blocks = f.read().strip().split('\n\n')

    blocks = [block.split() for block in _blocks]
    # print(blocks)
    cols = [palindromes(block) for block in blocks]
    blocks_t = [transpose(block) for block in blocks]
    rows = [palindromes(block) for block in blocks_t]
    # print(rows)

    print(sum(sum(row) for row in rows) + 100 * sum(sum(col) for col in cols))