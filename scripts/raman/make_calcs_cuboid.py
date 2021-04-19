import numpy as np
import random
import csv


n = 10

xrange = (-0.02, 0.02)
yrange = (-0.0005, 0.0005)
zrange = (-0.0005, 0.0005)

f = open("input/og_cuboid_calc.csv", "w")

points = []

[points.append((random.uniform(*xrange), random.uniform(*yrange), random.uniform(*zrange))) for i in range(n)]
print(points)

with f as points_file:
    for i in points:
        points_writer = csv.writer(points_file, delimiter=',')
        points_writer.writerow(i)
