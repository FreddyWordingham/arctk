import random
import math
import numpy as np


class Perlin:
    grid = []

    def __init__(self, nx, ny):
        assert(nx >= 2)
        assert(ny >= 2)

        self.grid = np.empty((nx, ny, 2))

        for iy in range(ny):
            for ix in range(nx):
                theta = random.uniform(0.0, 2.0 * math.pi)
                self.grid[ix, iy, 0] = math.sin(theta)
                self.grid[ix, iy, 1] = math.cos(theta)

    def sample(self, x, y):
        (nx, ny, _) = self.grid.shape

        # Move point inside the grid.
        x = math.modf(x)[0]
        y = math.modf(y)[0]

        # Indices
        ix0 = math.floor(x * nx)
        ix1 = (ix0 + 1) % nx
        iy0 = math.floor(y * ny)
        iy1 = (iy0 + 1) % ny

        # UV coordinates
        u = 1.0 - (x * nx) + ix0
        v = 1.0 - (y * ny) + iy0

        # Gradient vectors dot position vectors
        g00 = self.grid[ix0, iy0, :].dot([u, v])
        g10 = self.grid[ix1, iy0, :].dot([1.0 - u, v])
        g01 = self.grid[ix0, iy1, :].dot([u, 1.0 - v])
        g11 = self.grid[ix1, iy1, :].dot([1.0 - u, 1.0 - v])

        # Interpolate
        a = interpolate(u, g00, g10)
        b = interpolate(u, g01, g11)
        c = interpolate(v, a, b)

        return c / math.sqrt(2)


def fade(t):
    return t*t*t*(t*(t*6.0 - 15.0) + 10.0)


def interpolate(x, a, b):
    x = fade(x)
    return (x * a) + ((1.0 - x) * b)
