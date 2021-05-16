import matplotlib.pyplot as plt
import matplotlib.image as img
import numpy as np
from progress.bar import Bar
from perlin import Perlin
import math


def prime_list(min, max):
    assert(min >= 2)
    assert(max > min)

    primes = []
    for n in range(min, max + 1):
        for i in range(2, n):
            if (n % i) == 0:
                break
        else:
            primes.append(n)

    return primes


def plot(data, t):
    print("Plotting")
    print("Min: ", data.min())
    print("Max: ", data.max())

    fig = plt.figure(figsize=(6, 3.2))
    # plt.set_cmap('RdPu')
    # plt.set_cmap('gist_rainbow')

    ax = fig.add_subplot(111)
    plt.imshow(data)
    ax.set_aspect('equal')

    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
    ax.patch.set_alpha(0)
    ax.set_frame_on(False)

    plt.savefig(f"output/noise_{t:03}.png", bbox_inches='tight')


def sample(noise, ratios, x, y):
    sum = 0.0
    for (n, r) in zip(noise, ratios):
        sum += n.sample(x, y) * r
    return sum


def __main__():
    p = 10
    nx = 2 ** p
    ny = 2 ** p
    sx = 1.0
    sy = 1.0
    fx = sx / nx
    fy = sy / ny
    powers = prime_list(10, 30)

    steps = 1000
    rate = 0.25

    levels = 4
    inv_levels = 1.0 / levels

    print(f"Resolution {nx} x {ny}")
    print(f"powers: {powers}")

    noise = []
    ratios = []
    for p in powers:
        noise.append(Perlin(p, int(p * ny / nx)))
        ratios.append(1.0 / p)

    for nt in range(steps):
        bar = Bar("Sampling", max=ny)
        samples = np.zeros([nx, ny])
        for iy in range(ny):
            y = iy * fy
            for ix in range(nx):
                x = ix * fx
                samples[ix, iy] = sample(noise, ratios, x, y)
            bar.next()
        bar.finish()

        min = samples.min()
        max = samples.max()
        print("Min: ", min)
        print("Max: ", max)

        samples -= min
        samples /= (max - min)

        for iy in range(ny):
            for ix in range(nx):
                samples[ix, iy] = (samples[ix, iy] % inv_levels) * levels

        plot(samples, nt)
        for (n_map, r) in zip(noise, ratios):
            n_map.evolve(r * rate)


__main__()
