from math import inf

maps = []

with open('input8.txt') as f:
    data = f.read()
    sections = data.strip().split('\n\n')
    
    seeds = [int(i) for i in sections[0].split(':')[1].split()]
    # print(seeds)

    maps = []
    for sc in sections[1:]:
        map = []
        range_strs = sc.strip().split('\n')[1:]
        for range_str in range_strs:
            dest_start, src_start, range_size = [int(i) for i in range_str.split()]
            # print(range(src_start, src_start + range_size), dest_start)
            map.append((range(src_start, src_start + range_size), dest_start))
        maps.append(map)
    
    min_location = inf
    for seed in seeds:
        # print('seed', seed)
        pos = seed
        for map in maps:
            found = False
            for rng, dest in map:
                if pos in rng:
                    # print('found map', rng.start, dest, len(rng))
                    next_pos = (pos - rng.start) + dest
                    found = True
                    break
            if not found:
                next_pos = pos
            # print('pos is now', next_pos)
            pos = next_pos
        min_location = min(min_location, pos)
    print(min_location)
