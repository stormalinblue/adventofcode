from math import *

d = 214117714021024
t = 46807866

dcm = (t ** 2 - 4 * d) ** 0.5
ans1 = (t + dcm) / 2
ans2 = (t - dcm) / 2
print(ans1, ans2)
print(- ceil(ans2) + floor(ans1) + 1)