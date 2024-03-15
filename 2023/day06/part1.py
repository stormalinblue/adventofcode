challenges = [
    (46, 214),
    (80, 1177),
    (78, 1402),
    (66, 1024)
]

prod_ways_to_win = 1
for (time, distance) in challenges:
    print('time', time, 'distance', distance)
    ways_to_win = 0
    for acc_time in range(time + 1):
        vel = acc_time * 1
        travel_distance = vel * (time - acc_time)
        if travel_distance > distance:
            ways_to_win += 1
            print('can win by waiting', acc_time, 'travelling', travel_distance, 'in remaining time', (time - acc_time))
    prod_ways_to_win *= ways_to_win

print(prod_ways_to_win)
