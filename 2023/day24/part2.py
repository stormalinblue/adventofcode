import math
import numpy
import scipy
np = numpy

def mul(vec, t):
    return tuple(t * i for i in vec)

def add(a, b):
    return tuple(c + d for c, d in zip(a, b))

def within(a, b, x):
    return all((i <= j <= k) or (k <= j <= i) for i, j, k in zip(a, x, b))

def normalize(x):
    # print(x)
    return x/numpy.sqrt((x ** 2).sum())

def does_intersect(rock_a, rock_b):
    A = numpy.stack([rock_a[1].T, -rock_b[1]], axis=-1)
    B = rock_b[0] - rock_a[0]
    # print(rock_a[1], rock_b[1])
    # print(A, B)
    # print(A[:2,:], B[:2])
    # print(A[1:,:], B[1:])
    s1 = numpy.linalg.solve(A[:2,:], B[:2])
    s2 = numpy.linalg.solve(A[1:,:], B[1:])
    s3 = numpy.linalg.solve(A[0::2,:], B[::2])
    return (s1, s2, s3)

def intersect_times(rock_a, rock_b):
    A = numpy.stack([rock_a[1].T, -rock_b[1]], axis=-1)
    B = rock_b[0] - rock_a[0]
    # print(rock_a[1], rock_b[1])
    # print(A, B)
    # print(A[:2,:], B[:2])
    # print(A[1:,:], B[1:])
    s1 = numpy.linalg.solve(A[:2,:], B[:2])
    s2 = numpy.linalg.solve(A[1:,:], B[1:])
    if s1[1] == s2[0]:
        return numpy.array(s1[0], s1[1], s2[1])
    else:
        raise ValueError

def project_rock(rock, proj):
    return (rock[0] - numpy.dot(rock[0], proj) * proj,
            rock[1] - numpy.dot(rock[1], proj) * proj)

def magnitude(v):
    return numpy.sqrt((v ** 2).sum())

def project_xy(vec):
    return numpy.array([vec[0], vec[1], 0])

def project_yz(vec):
    return numpy.array([0, vec[1], vec[2]])

def project_zx(vec):
    return numpy.array([vec[0], 0, vec[2]])

def extrapolate_rock(rock, time):
    return rock[0] + time * rock[1]

def mul_rock(rock, mul):
    return (rock[0] * mul, rock[1] * mul)

def sub_rock(rock, d):
    return (rock[0] - d, rock[1])

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

    for rock in rocks:
        print(rock)

    sel_rocks = rocks[:3]
    sel_rocks = [sub_rock(r,numpy.array([321232980504586, 243177499261375, 187077988468444])) for r in sel_rocks]

    def closest_approach(a, b):
        p1, d1 = a
        p2, d2 = b
        n = numpy.cross(d1, d2)
        n1 = numpy.cross(d1, n)
        n2 = numpy.cross(d2, n)
        c1 = p1 +  d1 * numpy.dot((p2 - p1), n2) / numpy.dot(d1, n2)
        c2 = p2 + d2 * numpy.dot((p2 - p1), n1) / numpy.dot(d2, n1)
        d = c1 - c2
        return (d ** 2).sum()
    
    def loss_fn(x):
        rock = (numpy.array((x[0], x[1], x[2])), normalize(numpy.array((x[3], x[4], x[5]))))
        return numpy.array([closest_approach(rock, r) for r in sel_rocks])
    
    def stupid_loss_fn(x):
        rock = (numpy.array(x[:3]), numpy.array(x[3:6]))
        times = x[6:]
        return magnitude(numpy.array([magnitude(extrapolate_rock(rock, t) - extrapolate_rock(r, t)) for r, t in zip(sel_rocks, times)]))
    
    def real_loss_fn(x):
        p0, v0 = (numpy.arra(x[:3]), numpy.array(x[3:6]))
        p1, v1 = rocks[0]
        p2, v2 = rocks[1]
        p3, v3 = rocks[2]

        lhs1 = np.cross(p1, v1) - np.cross(p0, v1) - np.cross(p1, v0)
        rhs1 = np.cross(p2, v2) - np.cross(p0, v2) - np.cross(p2, v0)
        rhs2 = np.cross(p3, v3) - np.cross(p0, v3) - np.cross(p3, v0)

        return magnitude(lhs1 - rhs1) + magnitude(lhs1 - rhs2) 
    
    from scipy.optimize import least_squares, minimize, root_scalar

    # sol = least_squares(stupid_loss_fn, [0 for _ in range(6 + len(sel_rocks))], method='Nelder-Mead', options={'adaptive':True, 'xatol':1e-23})
    sol = least_squares(stupid_loss_fn, [1 for _ in range(6)], ftol=None, xtol=None, max_nfev=100000)
    print(sol)
    print(loss_fn(sol.x))
#     intersecting_rocks = []
    
#     for i in range(len(rocks)):
#         rock_a = rocks[i]
#         # print(rock_a)
#         for j in range(i + 1, len(rocks)):
#             rock_b = rocks[j]
#             # print(rock_a, rock_b)

#             # print(numpy.array([[a, b], [c, d]]))
#             try:
#                 sol = does_intersect(rock_a, rock_b)
#                 rock_a_proj = [
# project_xy(extrapolate_rock(rock_a, sol[0][0])),
# project_yz(extrapolate_rock(rock_a, sol[1][0])),
# project_zx(extrapolate_rock(rock_a, sol[2][0]))
#                 ]
#                 rock_b_proj = [
# project_xy(extrapolate_rock(rock_b, sol[0][1])),
# project_yz(extrapolate_rock(rock_b, sol[1][1])),
# project_zx(extrapolate_rock(rock_b, sol[2][1]))
#                 ]

#                 print('int proj a', rock_a_proj)
#                 print('int proj b', rock_b_proj)
#                 if sol:
#                     intersecting_rocks.append((rock_a, rock_b, rock_a_proj))
#             except numpy.linalg.LinAlgError:
#                 pass

#             # if dvx - dvy == 0:
#             #     if dpx == 0 and dpy == 0:
#             #         if within(test_area_start, test_area_end, rock_a[0]):
#             #             count_intersect += 1
#             #     else:
#             #         print(f'{i} {j} parallel')
#             #         continue

#             # t = (dpy - dpx) / (dvx - dvy)
#             # print(f'{dvx}t + {dpx} = 0' + "\n" + f'{dvy}t + {dpy} = 0')
#             # print('intersect at ', t)
#             # if t >= 0:
#             #     collide_pos = add(rock_a[0], mul(rock_a[1], t))
#             #     print('intersect at pos ', collide_pos)
#             #     if within(test_area_start, test_area_end, collide_pos):
#             #         print('inside')
#             #         count_intersect += 1
#             #     else:
#             #         print('outside')
    


#     for i in intersecting_rocks:
#         print(i)
    
    # interceptor_vel = (rocks[0][0] + times[0] * rocks[0][1] - rocks[1][0] - times[1] * rocks[1][1]) / (times[0] - times[1])
    # print(interceptor_vel, normalize(interceptor_vel), vel_n)

    # interceptor_pos = rocks[0][0] + times[0] * rocks[0][1] - interceptor_vel * times[0]
    # print(interceptor_pos)
    # print(interceptor_pos.sum())
    # print(int(interceptor_pos.sum()))

    # for rock in rocks:
    #     print(intersect_times((interceptor_pos, interceptor_vel), rock))
    
    # print('num intersect', count_intersect)
            
