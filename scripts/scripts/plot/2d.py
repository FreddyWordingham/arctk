import matplotlib.pyplot as plt
import numpy as np
import sys


X_AXIS_LABEL = 'x_axis_label'
Y_AXIS_LABEL = 'y_axis_label'


def quit_figure(event):
    if event.key == 'escape':
        plt.close(event.canvas.figure)


cid = plt.gcf().canvas.mpl_connect('key_press_event', quit_figure)

filename = sys.argv[1]
print("Loading file", filename)

data = np.genfromtxt(filename, delimiter=',', names=['x', 'y'])
plt.plot(data['x'], data['y'])

plt.xlabel(X_AXIS_LABEL)
plt.ylabel(Y_AXIS_LABEL)
plt.title(filename)
plt.legend()

plt.show()
plt.close()
