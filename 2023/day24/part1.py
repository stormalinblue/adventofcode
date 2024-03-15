import math
import numpy

def mul(vec, t):
    return tuple(t * i for i in vec)

def add(a, b):
    return tuple(c + d for c, d in zip(a, b))

def within(a, b, x):
    return all((i <= j <= k) or (k <= j <= i) for i, j, k in zip(a, x, b))

with open('input24.txt') as f:
    rocks = []
    
    for line in f:
        pos_s, vel_s = line.split(' @ ')
        pos = numpy.array(list(int(i) for i in pos_s.split(',')))
        vel = numpy.array(list(int(i) for i in vel_s.split(',')))
        rocks.append((pos, vel))
    
    small = 200000000000000
    big = 400000000000000
    # small = 7
    # big = 27
    test_area_start = (small, small, -math.inf)
    test_area_end = (big, big, math.inf)

    count_intersect = 0
    
    
    for i in range(len(rocks)):
        rock_a = rocks[i]
        # print(rock_a)
        for j in range(i + 1, len(rocks)):
            rock_b = rocks[j]
            # print(rock_a, rock_b)

            a = rock_a[1][0]
            b = -rock_b[1][0]
            c = rock_a[1][1]
            d = -rock_b[1][1]

            e = rock_b[0][0] - rock_a[0][0]
            f = rock_b[0][1] - rock_a[0][1]

            # print(numpy.array([[a, b], [c, d]]))
            try:
                sol = numpy.linalg.solve([[a, b], [c, d]], [e, f])
                if all(x >= 0 for x in sol) and within(test_area_start, test_area_end, rock_a[0] + rock_a[1] * sol[0]):
                    count_intersect += 1
            except numpy.linalg.LinAlgError:
                pass

            # if dvx - dvy == 0:
            #     if dpx == 0 and dpy == 0:
            #         if within(test_area_start, test_area_end, rock_a[0]):
            #             count_intersect += 1
            #     else:
            #         print(f'{i} {j} parallel')
            #         continue

            # t = (dpy - dpx) / (dvx - dvy)
            # print(f'{dvx}t + {dpx} = 0' + "\n" + f'{dvy}t + {dpy} = 0')
            # print('intersect at ', t)
            # if t >= 0:
            #     collide_pos = add(rock_a[0], mul(rock_a[1], t))
            #     print('intersect at pos ', collide_pos)
            #     if within(test_area_start, test_area_end, collide_pos):
            #         print('inside')
            #         count_intersect += 1
            #     else:
            #         print('outside')
    
    print('num intersect', count_intersect)
            
