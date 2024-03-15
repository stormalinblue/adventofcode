from collections import Counter

def is_five_of_kind(hand):
    return all(c == hand[0] for c in hand)

card_level = {
    'J': 0,
    '2': 1,
    '3': 2,
    '4': 3,
    '5': 4,
    '6': 5,
    '7': 6,
    '8': 7,
    '9': 8,
    'T': 9,
    'Q': 11,
    'K': 12,
    'A': 13
}
def hand_key(hand, card_levels):
    counter = Counter(hand)
    mc = counter.most_common()
    # card_levels = [card_level[c] for c in hand]
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

def hand_key2(hand):
    counter = Counter(hand)
    mc = [d[1] for d in sorted([((c[1], card_level[c[0]]), c) for c in counter.most_common()], reverse=True)]
    i = 0 if mc[0][0] != 'J' else min(len(mc) - 1, 1)
    new_hand = hand.replace('J', mc[i][0])
    card_levels = [card_level[c] for c in hand]
    # print(hand, new_hand, mc)

    return hand_key(new_hand, card_levels)

with open('input12.txt') as f:
    hand_bids = []
    for line in f:
        hand, bid = line.strip().split()
        hand_bids.append((hand, int(bid)))
    
    # print(hand_bids)
    hand_bids.sort(key=lambda x: hand_key2(x[0]))
    # print(hand_bids)
    # print('\n'.join([str((b, hand_key2(b[0]))) for b in hand_bids]))

    s = 0
    for i, j in enumerate(hand_bids):
        rank = i + 1
        bid = j[1]
        s += rank * bid
    print(s)
