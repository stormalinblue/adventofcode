from math import inf

maps = []

with open('input8.txt') as f:
    data = f.read()
    sections = data.strip().split('\n\n')
    
    seed_digits = [int(i) for i in sections[0].split(':')[1].split()]
    seed_ranges = [range(seed_digits[2 * i], seed_digits[2 * i] + seed_digits[2 * i + 1]) for i in range(len(seed_digits) // 2)]
    # print(seeds)
    print(seed_ranges)

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
    step_seed_ranges = list(seed_ranges)
    for map in maps:
        # print('map start')
        # new_unassigned = list(step_seed_ranges)
        unassigned = list(step_seed_ranges)
        assigned = []
        for src_rng, dest in map:
            # print('src_rng', src_rng)
            # print('unassigned', unassigned)
            offset = dest - src_rng.start
            new_unassigned = []
            for rng in unassigned:
                # print('unassigned range', rng)
                if rng.start >= src_rng.stop or rng.stop <= src_rng.start:
                    # print('completely outside')
                    new_unassigned.append(rng)
                elif rng.start <= src_rng.start and rng.stop >= src_rng.stop:
                    # print('bigger on both sides')
                    assigned.append(range(src_rng.start + offset, src_rng.stop + offset))
                    new_unassigned.append(range(rng.start, src_rng.start))
                    new_unassigned.append(range(src_rng.stop, rng.stop))
                elif rng.start <= src_rng.start and rng.stop < src_rng.stop:
                    # print('bigger on left')
                    assigned.append(range(src_rng.start + offset, rng.stop + offset))
                    new_unassigned.append(range(rng.start, src_rng.start))
                elif rng.start > src_rng.start and rng.stop >= src_rng.stop:
                    # print('bigger on right')
                    assigned.append(range(rng.start + offset, src_rng.stop + offset))
                    new_unassigned.append(range(src_rng.stop, rng.stop))
                elif rng.start > src_rng.start and rng.stop < src_rng.stop:
                    # print('completely within')
                    assigned.append(range(rng.start + offset, rng.stop + offset))
                else:
                    print('SCREAM!!!')
                    raise ValueError
                unassigned = new_unassigned
        step_seed_ranges = [l for l in assigned + unassigned if len(l) > 0]
        # print('step seed ranges', step_seed_ranges)

    print(min(r.start for r in step_seed_ranges if len(r) > 0))
