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


def __main__():
    print("Hello arctk!")

    l = 10
    p = 12
    # ny = 1440
    # nx = 900
    nx = 2 ** p
    ny = 2 ** p
    nt = 2000
    fx = 1.0 / nx
    fy = 1.0 / ny

    # powers = [2, 3, 5, 7, 11, 13, 17, 19, 23, 27, ]
    powers = prime_list(2, 9)
    print(f"powers: {powers}")
    noise = []
    ratios = []
    for p in powers:
        noise.append(Perlin(p, int(p * ny / nx)))
        ratios.append(1.0 / p)

    samples = np.zeros([nx, ny])
    dt = 0.25
    for t in range(nt):
        bar = Bar("Sampling", max=ny)
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

        # plt.hist(samples.flatten(), bins=1000)
        # plt.title("histogram")
        # plt.show()

        for iy in range(ny):
            for ix in range(nx):
                z = samples[ix, iy]
        #         if z < 0.495:
        #             samples[ix, iy] = 0.0
        #         if z > 0.505:
        #             samples[ix, iy] = 0.0
        #         else:
                z = math.fmod(z, 1.0 / l)
                samples[ix, iy] = z

        fig = plt.figure(figsize=(6, 3.2))

        ax = fig.add_subplot(111)
        plt.imshow(samples)
        ax.set_aspect('equal')

        ax.get_xaxis().set_visible(False)
        ax.get_yaxis().set_visible(False)
        ax.patch.set_alpha(0)
        ax.set_frame_on(False)
        plt.savefig(f"output/noise_{t:03}.png", bbox_inches='tight')

        for (n, r) in zip(noise, ratios):
            # n.update(dt * r)
            n.update(dt)


def sample(noise, ratios, x, y):
    t = 0.0
    for (n, r) in zip(noise, ratios):
        t += r * n.sample(x, y)
    return t


__main__()
