import numpy as np
import scipy
from matplotlib import pyplot as plt

pos = range(-8, 9)
pos2 = range(-5, 6)
fig, axs = plt.subplots(3)

small_sphere = []
large_sphere = []
cuboid = []
f = open("output/testing/Raman_weight_power_sphere_2mm_10dist.txt", 'r')
for line in f:
    small_sphere.append(float(line))
f = open("output/testing/Raman_weight_power_sphere_1cm_10dist.txt", 'r')
for line in f:
    large_sphere.append(float(line))
f = open("output/testing/Raman_weight_power_cuboid_10dist.txt", 'r')
for line in f:
    cuboid.append(float(line))


mean_small = []
sd_small = []
full_small = []
mean_cuboid = []
sd_cuboid = []
full_cuboid = []
for i in range(0, len(small_sphere), 17):
    group_sphere = small_sphere[i:i+17]
    full_small.append(group_sphere)
    group_cuboid = cuboid[i:i+17]
    full_cuboid.append(group_cuboid)
    axs[0].plot(pos, group_sphere)
    axs[2].plot(pos, group_cuboid)

mean_large = []
sd_large = []
full_large = []
for i in range(0, len(large_sphere), 11):
    group_sphere = large_sphere[i:i+11]
    if large_sphere[i+10] > 2.5e-8:
        axs[1].plot(pos2, group_sphere)
        continue
    else:
        full_large.append(group_sphere)
        axs[1].plot(pos2, group_sphere)
plt.show()

#group_small = []
for i in range(len(full_small[0])):
    group_small = []
    group_cuboid = []
    for j in range(len(full_small)):
        group_small.append(full_small[j][i])
        group_cuboid.append(full_cuboid[j][i])
    mean_small.append(np.mean(group_small))
    sd_small.append(np.std(group_small))
    mean_cuboid.append(np.mean(group_cuboid))
    sd_cuboid.append(np.std(group_cuboid))

for i in range(len(full_large[0])):
    group_large = []
    for j in range(len(full_large)):
        group_large.append(full_large[j][i])
    mean_large.append(np.mean(group_large))
    sd_large.append(np.std(group_large))

plt.errorbar(pos, mean_small, xerr=None, yerr=sd_small, label="2mm sphere")
#plt.errorbar(pos2, mean_large, xerr=None, yerr=sd_large, label="1cm sphere")
#plt.errorbar(pos, mean_cuboid, xerr=None, yerr=sd_cuboid, label="1mm cuboid")
plt.ylim(0,)
plt.legend()
plt.show()
