import matplotlib.pyplot as plt
import matplotlib.image as img
import numpy as np
from progress.bar import Bar
from perlin import Perlin
import math


def __main__():
    print("Hello arctk!")

    powers = [10, 20, 4, 234, 68, 9, 192, 93]
    noise = []
    ratios = []
    for p in powers:
        noise.append(Perlin(p, p))
        ratios.append(1.0 / p)

    l = 7
    p = 8
    nx = 2 ** p
    ny = 2 ** p
    fx = 1.0 / nx
    fy = 1.0 / ny

    samples = np.zeros([nx, ny])
    dt = 0.1
    for t in range(1000):
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

        for iy in range(ny):
            for ix in range(nx):
                x = samples[ix, iy]
                x = math.floor(x * l) / l
                samples[ix, iy] = x

        # samples = np.vectorize(step)(samples)

        fig = plt.figure(figsize=(6, 3.2))

        ax = fig.add_subplot(111)
        plt.imshow(samples)
        ax.set_aspect('equal')

        ax.get_xaxis().set_visible(False)
        ax.get_yaxis().set_visible(False)
        ax.patch.set_alpha(0)
        ax.set_frame_on(False)
        plt.savefig(f"output/noise_{t:03}.png", bbox_inches='tight')

        for n in noise:
            n.update(dt)


def sample(noise, ratios, x, y):
    t = 0.0
    for (n, r) in zip(noise, ratios):
        t += r * n.sample(x, y)
    return t


def step(x):
    2.0 * x


__main__()
