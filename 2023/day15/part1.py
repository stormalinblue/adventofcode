def hsh(s):
    h = 0
    for c in s:
        h += ord(c)
        h *= 17
        h %= 256
    return h

with open('input15.txt') as f:
    print(hsh('rn=1'))

    s = 0
    for step in f.read().strip().split(','):
        s += hsh(step)
    print(s)