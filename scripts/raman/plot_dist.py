import matplotlib.pyplot as plt
import numpy as np
import scipy

raw_raman = []
raw_weights = []
pos = range(-8, 9)

f = open('/Users/lm579/Projects/arctk/output/Raman_weight.txt','r')
for line in f:
    #a = line.split(',')
    raw_raman.append(float(line))

if raw_raman[0] == 0:
    raw_raman = raw_raman[1:]
means = []
sd = []

if len(raw_raman) > 18:
    for i in range(0, len(raw_raman), 10):
        group = raw_raman[i:i+10]
        means.append(np.mean(group))
        sd.append(np.std(group))

percentage = [100*(a/b) for a, b in zip(sd, means)]
plt.plot(pos, means, color="blue", label="2mm detector")

    #plt.errorbar(pos, means, yerr=sd, marker='o', label='2mm detector', markersize=2)

#else:
#    raw_raman = [raw_raman[i]/np.min(raw_raman) for i in range(len(raw_raman))]
    #plt.plot(pos, raw_raman, marker='o')
    #plt.plot(pos, combo, linestyle='--', marker='o')



nopeel_raw_raman = []
f = open('/Users/lm579/Projects/arctk/output/Raman_weight_debug_2cm_170.txt','r')
for line in f:
    nopeel_raw_raman.append(float(line))
if nopeel_raw_raman[0]==0:
    nopeel_raw_raman = nopeel_raw_raman[1:]

nopeel_means = []
nopeel_sd = []

if len(nopeel_raw_raman) > 18:
    for i in range(0, len(nopeel_raw_raman), 10):
        group = nopeel_raw_raman[i:i+10]
        nopeel_means.append(np.mean(group))
        nopeel_sd.append(np.std(group))
percentage = [100*(a/b) for a, b in zip(nopeel_sd, nopeel_means)]
plt.plot(pos, nopeel_means, color='purple', label="2cm detector")
plt.legend()
plt.show()

    #plt.errorbar(pos, nopeel_means, yerr=nopeel_sd, marker='x', label='2cm detector', markersize=2)

#else:
#    nopeel_raw_raman = [nopeel_raw_raman[i]/np.min(nopeel_raw_raman) for i in range(len(nopeel_raw_raman))]
#    plt.plot(pos, nopeel_raw_raman, marker='x')
    #plt.plot(pos, combo_nopeel, linestyle='--', marker='x')



#plt.legend()
#plt.show()


bug = []
f = open('/Users/lm579/Projects/arctk/output/Raman_power_bug.txt','r')
for line in f:
    bug.append(float(line))
if bug[0]==0:
    bug = bug[1:]

#plt.plot(pos, bug, marker='x', markersize=2, label='bug')

bug_weights = []
f = open('/Users/lm579/Projects/arctk/output/weightings_debug_2cm_170.txt', 'r')
for line in f:
    bug_weights.append(float(line))
bug_energy = []
f = open('/Users/lm579/Projects/arctk/output/weightings.txt', 'r')
for line in f:
    bug_energy.append(float(line))

bug_power_2mm = [a*b for a, b in zip(means, bug_energy)]
plt.plot(pos, bug_power_2mm, marker='o', markersize=2, label="2mm detector", color='blue')
plt.ylim(0,)
plt.legend()
plt.show()
bug_power_2cm = [a*b for a, b in zip(nopeel_means, bug_weights)]

#plt.plot(pos, bug_power_2mm, marker='o', markersize=2, label="2mm detector", color='blue')
plt.plot(pos, bug_power_2cm, marker='x', markersize=2, label="2cm detector", color='purple')
plt.ylim(0,)
#plt.plot(pos, bug_energy, marker='o', markersize=2, label='2mm detector', color='blue')
#plt.plot(pos, bug_weights, marker='x', markersize=2, label='2cm detector', color='purple')

plt.legend()
plt.show()
