def parse_line(line):
    _, r = line.strip().split(':')
    num_win_str, num_strs_have_str = r.split('|')
    nums_win = set(int(i) for i in num_win_str.strip().split())
    nums_have = set(int(i) for i in num_strs_have_str.strip().split())
    return (nums_win, nums_have)

with open('input7.txt') as f:
    s = 0
    num_commons = []
    for line in f:
        nums_win, nums_have = parse_line(line)
        num_common = len(nums_win.intersection(nums_have))
        num_commons.append(num_common)
    
    num_cards = [1 for _ in num_commons]
    for i in range(len(num_commons)):
        nc = num_commons[i]
        for j in range(i + 1, min(i + 1 + nc, len(num_commons))):
            num_cards[j] += num_cards[i]
    
    print(num_commons)
    print(num_cards)

    print(sum(num_cards))