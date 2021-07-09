from random import sample
import matplotlib.pyplot as plt
import numpy as np
import math
from progress.bar import Bar

from script.arctk.rng.smooth import *
from script.arctk.rng.perlin_stack import PerlinStack
from script.arctk.rng.perlin import Perlin


# Noise settings
cell_res = [[7, 11], [6, 9]]
noise_res = [[23, 25], [31, 33], [43, 45], [38, 37], [32, 33]]
repeat = [1, 1]

# Sampling settings
power = 7
sample_res = [2**power, 2**power]

# Time step settings
dt = 1.0e-1
frames = 1000


def is_it_a_cell(t):
    a = 0.35
    b = 0.35
    if t > a:
        return t
    # if t < -b:
    #     return t
    return 0


if __name__ == "__main__":
    cells = []
    cells.append(PerlinStack(cell_res, smoothstep_5))
    cells.append(PerlinStack(cell_res, smoothstep_5))
    cells.append(PerlinStack(cell_res, smoothstep_5))
    cells.append(PerlinStack(cell_res, smoothstep_5))
    cells.append(PerlinStack(cell_res, smoothstep_5))
    noise = PerlinStack(noise_res, smoothstep_5)
    cycle = [0.1, 1.2, 2.0, 4.0, 5.0]
    freq = [1.0, 1.2, 1.4, 2.0, 3.0]

    time = 0.0
    for frame in range(frames):
        samples = np.zeros(sample_res)
        dx = repeat[0] / sample_res[0]
        dy = repeat[1] / sample_res[1]

        with Bar('Sampling', max=(sample_res[0] * sample_res[1])) as bar:
            for iy in range(sample_res[1]):
                sy = dy * iy
                for ix in range(sample_res[0]):
                    sx = dx * ix

                    # Background noise
                    n = noise.sample([sx, sy])

                    # Cells
                    t = 0
                    for (c, offset, f) in zip(cells, cycle, freq):
                        t += is_it_a_cell(c.sample([sx, sy])) * \
                            max(0, math.sin(offset + (time * f)))

                    samples[ix, iy] = t + (0.5 * n)
                    bar.next()

        samples[0, 0] = 0
        samples[0, 1] = 1
        print(samples.min(), samples.max())

        fig = plt.figure(figsize=(6, 3.2))
        ax = fig.add_subplot(111)
        plt.imshow(samples, cmap='gray')
        ax.set_aspect('equal')

        ax.get_xaxis().set_visible(False)
        ax.get_yaxis().set_visible(False)
        ax.patch.set_alpha(0)
        ax.set_frame_on(False)

        filename = f"output/frame_{frame:03}.png"
        print(f"Save: {filename}")
        plt.savefig(filename, bbox_inches='tight')

        time += dt
        for c in cells:
            c.rotate(dt * 0.1 * math.sin(time))
        noise.rotate(dt * 10.0)
