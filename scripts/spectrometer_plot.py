#!/usr/bin/python3


import pandas
import matplotlib.pyplot as plt
import sys


def quit_figure(event):
    if event.key == 'escape':
        plt.close(event.canvas.figure)


cid = plt.gcf().canvas.mpl_connect('key_press_event', quit_figure)


#   Settings
TITLE_LABEL = "Spectral Intensity"
X_AXIS_LABEL = "Wavelength (m)"
Y_AXIS_LABEL = "Count"

COLS_SMALL = ["r.",
              "m.",
              "b.",
              "c.",
              "g.",
              "y.",
              "k."]
COLS_LARGE = ["#FF0000",
              "#800000",
              "#FFFF00",
              "#808000",
              "#00FF00",
              "#008000",
              "#00FFFF",
              "#008080",
              "#0000FF",
              "#000080",
              "#FF00FF",
              "#800080",
              "#000000",
              "#808080",
              "#C0C0C0"]


#   Main
if len(sys.argv) != 2:
    print("Incorrect arguments: <filename>")
    quit()

filename = sys.argv[1]

# colnames = ['t', 'a', 'b', 'c', 'd', 'e', 'f', 'g']
data = pandas.read_csv(filename, header=0)
headers = list(pandas.read_csv(filename, header=0).head())

num_specs = len(data.columns) - 1
print("Total species: ", num_specs)

if num_specs <= len(COLS_SMALL):
    cols = COLS_SMALL
else:
    cols = COLS_LARGE

col_number = 0
for col in data:
    if col_number == 0:
        y = data[col]
    else:
        plt.plot(y, data[col], cols[(col_number % len(cols)) - 1])
    col_number += 1

plt.xlabel(X_AXIS_LABEL)
plt.ylabel(Y_AXIS_LABEL)
plt.title(TITLE_LABEL)
plt.legend(headers[1:])

plt.show()
plt.close()
