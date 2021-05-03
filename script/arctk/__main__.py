import matplotlib.pyplot as plt
import matplotlib.image as img
import numpy as np
from progress.bar import Bar
from perlin import Perlin


def __main__():
    print("Hello arctk!")

    powers = [5, 10, 20, 4, 234, 68, 3, 8, 6, 9, 192, 93]
    noise = []
    ratios = []
    for p in powers:
        noise.append(Perlin(p, p))
        ratios.append(1.0 / p)

    nx = 100
    ny = 100
    fx = 1.0 / nx
    fy = 1.0 / ny

    samples = np.zeros([nx, ny])
    bar = Bar("Sampling", max=nx * ny)
    for iy in range(ny):
        y = iy * fy
        for ix in range(nx):
            x = ix * fx
            samples[ix, iy] = sample(noise, ratios, x, y)
            bar.next()
    bar.finish()

    min = samples.min()
    max = samples.max()
    samples = (samples - min) / (max - min)
    samples = np.vectorize(clamp)(samples)
    samples *= 256

    fig = plt.figure(figsize=(6, 3.2))

    ax = fig.add_subplot(111)
    plt.imshow(samples)
    ax.set_aspect('equal')

    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
    ax.patch.set_alpha(0)
    ax.set_frame_on(False)
    plt.savefig("noise.png", bbox_inches='tight')


def sample(noise, ratios, x, y):
    t = 0.0
    for (n, r) in zip(noise, ratios):
        t += r * n.sample(x, y)
    return t


def clamp(x):
    if x < 0.25:
        return 0.25

    if x > 0.75:
        return 0.75

    return x


__main__()
