import numpy as np
from math import pi as PI
import math
from scipy.special import gammainc
from matplotlib import pyplot as plt
import csv
def sample(center,radius,n_per_sphere):
    r = radius
    ndim = center.size
    x = np.random.normal(size=(n_per_sphere, ndim))
    #print("x", x)
    ssq = np.sum(x**2,axis=1)
    #print("ssq", ssq)
    fr = r*gammainc(ndim/2,ssq/2)**(1/ndim)/np.sqrt(ssq)
    #print("fr", fr)
    frtiled = np.tile(fr.reshape(n_per_sphere,1),(1,ndim))
    #print("frtiled", frtiled)
    p = center + np.multiply(x,frtiled)
    return p

f = open("input/og_sphere_calc.csv", "w")
g = open("input/sample.txt", "w")
#fig1 = plt.figure(1)
#ax1 = fig1.gca()
#center = np.array([0,0,0])
#radius = 0.001
#p = sample(center,radius,10)
#print(p)
test = []
test_x = np.linspace(0, 1, 10)
#print(PI)
for i in range(1000):
    theta = np.random.uniform(0, 2*PI)
    v = np.random.uniform(0,1)
    phi = math.acos((2*v)-1)
    r = math.pow(np.random.uniform(0,1), 1/3)
    x=r*math.sin(phi)*math.cos(theta)
    y=r*math.sin(phi)*math.sin(theta)
    z=r*math.cos(phi)
    #print(x, y, z)
    #g.write(str([x, y, z]))
    test.append([x, y, z])
test = np.array(test)
#plt.scatter(test[:,0], test[:,1])
#plt.axes().set_aspect('equal')
#plt.show()
with f as points_file:
    for i in test:
        points_writer = csv.writer(points_file, delimiter=',')
        points_writer.writerow(i)
#ax1.scatter(p[:,0],p[:,1],s=0.5)
#ax1.add_artist(plt.Circle(center,radius,fill=False,color='0.5'))
#ax1.set_xlim(-1.5,1.5)
#ax1.set_ylim(-1.5,1.5)
#ax1.set_aspect('equal')
#plt.show()
