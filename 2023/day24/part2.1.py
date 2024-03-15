import numpy
np = numpy
import math

def magnitude(v):
    return numpy.sqrt((v ** 2).sum())

def extrapolate_rock(rock, time):
    return rock[0] + time * rock[1]

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

with open('input24.txt') as f:
    rocks = []
    
    for line in f:
        pos_s, vel_s = line.split(' @ ')
        pos = numpy.array(list(int(i) for i in pos_s.split(',')), dtype=numpy.int64)
        vel = numpy.array(list(int(i) for i in vel_s.split(',')), dtype=numpy.int64)
        rocks.append((pos, vel))


    p1, v1 = rocks[0]
    p2, v2 = rocks[1]
    p3, v3 = rocks[2]

    row1 = [
        0,
        -v1[2] + v2[2],
        v1[1] - v2[1],
        0,
        p1[2] - p2[2],
        - p1[1] + p2[1]
    ]

    row2 = [
        v1[2] - v2[2],
        0,
        -v1[0] + v2[0],
        -p1[2] + p2[2],
        0,
        p1[0] - p2[0]
    ]

    row3 = [
        -v1[1] + v2[1],
        v1[0] - v2[0],
        0,
        p1[1] - p2[1],
        -p1[0] + p2[0],
        0
    ]

    row4 = [
        0,
        -v1[2] + v3[2],
        v1[1] - v3[1],
        0,
        p1[2] - p3[2],
        - p1[1] + p3[1]
    ]

    row5 = [
        v1[2] - v3[2],
        0,
        -v1[0] + v3[0],
        -p1[2] + p3[2],
        0,
        p1[0] - p3[0]
    ]

    row6 = [
        -v1[1] + v3[1],
        v1[0] - v3[0],
        0,
        p1[1] - p3[1],
        -p1[0] + p3[0],
        0
    ]

    A = np.array([row1, row2, row3, row4, row5, row6])

    c1 = np.cross(p1, v1) - np.cross(p2, v2)
    c2 = np.cross(p1, v1) - np.cross(p3, v3)

    b = [-c1[0], -c1[1], -c1[2], -c2[0], -c2[1], -c2[2]]
    print(A, b)
    Rock = np.linalg.solve(A, b)
    print(Rock)

    print(int(sum(Rock[:3])))

    rock = (Rock[:3], Rock[3:])
    for r in rocks:
        print(does_intersect(rock, r))

    # print(loss_fn(sol.x))
    
