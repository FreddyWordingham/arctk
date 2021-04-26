import matplotlib.pyplot as plt
import numpy as np
import scipy

pos = range(-8, 9)
pos2 = range(-5, 6)
pos3 = [-2, -1.5, -1, -0.5, 0, 0.5, 1, 1.5, 2]

sphere_raw = []
f = open('/Users/lm579/Projects/arctk/output/Raman_weight_highres_2mm_sphere.txt','r')
for line in f:
    sphere_raw.append(float(line))

cuboid_raw = []
f = open('/Users/lm579/Projects/arctk/output/Raman_weight_highres_sphere_1cm.txt', 'r')
for line in f:
    cuboid_raw.append(float(line))

sphere_mean = []
sphere_sd = []
cuboid_mean = []
cuboid_sd = []
if len(sphere_raw) > 100:
    for i in range(0, len(sphere_raw), 10):
        sphere_group = sphere_raw[i:i+10]
    #cuboid_group = cuboid_raw[i:i+10]
        sphere_mean.append(np.mean(sphere_group))
        sphere_sd.append(np.std(sphere_group))
        plt.errorbar(pos, sphere_mean, xerr=None, yerr=sphere_sd, label="2mm diameter sphere")

    #cuboid_mean.append(np.mean(cuboid_group))
    #cuboid_sd.append(np.std(cuboid_group))
if len(cuboid_raw) > 100:
    for i  in range(0, len(cuboid_raw), 10):
        cuboid_group = cuboid_raw[i:i+10]
        cuboid_mean.append(np.mean(cuboid_group))
        cuboid_sd.append(np.std(cuboid_group))
        plt.errorbar(pos, cuboid_mean, xerr=None, yerr=cuboid_sd, label="1cm diameter sphere")
else:
    sphere_mean = sphere_raw
    cuboid_mean = cuboid_raw
    plt.plot(pos, sphere_raw, label="2mm diameter sphere")
    plt.plot(pos2, cuboid_raw, label="1cm diameter sphere")

plt.legend()
plt.show()
