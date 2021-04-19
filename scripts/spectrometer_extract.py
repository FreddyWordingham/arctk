import numpy as np

f = open('/Users/lm579/Projects/arctk/output/spectrometer.dat','r')

wavelength = []
count = []
for line in f:
    a = line.split(',')
    wavelength.append(a[0])
    count.append(float(a[1]))

nonzeroind = np.nonzero(count)[0]
print("non zero: ", nonzeroind)
print("count: ", count[nonzeroind[0]])

g = open('/Users/lm579/Projects/arctk/output/Raman_weight.txt', 'a')
g.write("{}\n".format(count[nonzeroind[0]]))
