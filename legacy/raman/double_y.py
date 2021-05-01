import matplotlib.pyplot as plt
import numpy as np
import scipy

pos = range(-8, 9)
pos2 = range(-5, 6)
pos3 = [-2, -1.5, -1, -0.5, 0, 0.5, 1, 1.5, 2]

fig,ax=plt.subplots()

sphere_raw = []
f = open('/Users/lm579/Projects/arctk/output/Raman_weight_highres_sphere_2mm.txt','r')
for line in f:
    sphere_raw.append(float(line))

cuboid_raw = []
f = open('/Users/lm579/Projects/arctk/output/Raman_weight_highres_sphere_1cm.txt', 'r')
for line in f:
    cuboid_raw.append(float(line))

weightings_1 = []
f = open('/Users/lm579/Projects/arctk/output/weightings_highres_sphere_2mm.txt', 'r')
for line in f:
    weightings_1.append(float(line))

weightings_2 = []
f = open('/Users/lm579/Projects/arctk/output/weightings_highres_sphere_1cm.txt', 'r')
for line in f:
    weightings_2.append(float(line))

sphere_mean = []
sphere_sd = []
cuboid_mean = []
cuboid_sd = []
if len(sphere_raw) > 100:
    for i in range(0, len(sphere_raw), 10):
        sphere_group = sphere_raw[i:i+10]
        sphere_mean.append(np.mean(sphere_group))
        sphere_sd.append(np.std(sphere_group))
        ax.errorbar(pos, sphere_mean, xerr=None, yerr=sphere_sd, label="2mm diameter sphere, detected power")

if len(cuboid_raw) > 100:
    for i  in range(0, len(cuboid_raw), 10):
        cuboid_group = cuboid_raw[i:i+10]
        cuboid_mean.append(np.mean(cuboid_group))
        cuboid_sd.append(np.std(cuboid_group))
        ax.errorbar(pos, cuboid_mean, xerr=None, yerr=cuboid_sd, label="1cm diameter sphere, detected power")
else:
    sphere_mean = sphere_raw
    cuboid_mean = cuboid_raw
    ax.plot(pos, sphere_raw, label="2mm diameter sphere, detected power", color="purple", marker='x')
    ax.plot(pos2, cuboid_raw, label="1cm diameter sphere, detected power", color="blue", marker='o')
    ax.set_xlabel("Centre of calcification distribution")
    ax.set_ylabel("Detected power of Raman photons")

sums_1 = []
sums_2 = []
for i in range(0,len(weightings_1),10):
    weightings_1_group = weightings_1[i:i+10]
    sums_1.append(np.sum(weightings_1_group))

for i in range(0,len(weightings_2), 10):
    weightings_2_group = weightings_2[i:i+10]
    sums_2.append(np.sum(weightings_2_group))

#second y axis / subplots

ax2 = ax.twinx()
ax2.plot(pos, sums_1, color="purple", linestyle='--', label="2mm diameter, total power")
ax2.plot(pos2, sums_2, color="blue", linestyle='--', label="1cm diameter, total power")
ax2.set_ylabel("Total power emitted")

ax.legend(bbox_to_anchor=(0.75, 1.15), ncol=4)
plt.show()
