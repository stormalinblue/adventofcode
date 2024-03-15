from collections import Counter

def is_five_of_kind(hand):
    return all(c == hand[0] for c in hand)

card_level = {
    '2': 1,
    '3': 2,
    '4': 3,
    '5': 4,
    '6': 5,
    '7': 6,
    '8': 7,
    '9': 8,
    'T': 9,
    'J': 10,
    'Q': 11,
    'K': 12,
    'A': 13
}
def hand_key(hand):
    counter = Counter(hand)
    mc = counter.most_common()
    card_levels = [card_level[c] for c in hand]
    if len(counter) == 1:
        return [7] + card_levels
    elif len(counter) == 2:
        if mc[0][1] == 4:
            return [6] + card_levels
        elif mc[0][1] == 3:
            # Full house
            return [5] + card_levels
        # print('scream', hand, mc)
    elif len(counter) == 3:
        if mc[0][1] == 3:
            return [4] + card_levels
        elif mc[0][1] == 2:
            return [3] + card_levels
        # print('scream2', hand)
    else:
        if mc[0][1] == 2:
            return [2] + card_levels
        else:
            return [1] + card_levels

with open('input12.txt') as f:
    hand_bids = []
    for line in f:
        hand, bid = line.strip().split()
        hand_bids.append((hand, int(bid)))
    
    # print(hand_bids)
    hand_bids.sort(key=lambda x: hand_key(x[0]))
    print(hand_bids)
    print('\n'.join([str((b, hand_key(b[0]))) for b in hand_bids]))

    s = 0
    for i, j in enumerate(hand_bids):
        rank = i + 1
        bid = j[1]
        s += rank * bid
    print(s)
