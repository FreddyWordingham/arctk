import numpy as np
from scipy.special import gammainc
from matplotlib import pyplot as plt
import csv

f = open("input/weightings.csv", "r")

weightings = open("output/weightings.txt", "a")

reader = csv.reader(f)
for i in reader:
    weightings.write("{}\n".format(float(i[0])))
