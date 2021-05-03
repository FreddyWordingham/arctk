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
        assert(x >= 0)
        assert(x <= 1)
        assert(y >= 0)
        assert(y <= 1)

        (nx, ny, _) = self.grid.shape
        dx = 1.0 / (nx - 1)
        dy = 1.0 / (ny - 1)

        ix = math.floor(x / dx)
        iy = math.floor(y / dy)

        u = (x - (ix * dx)) / dx
        v = (y - (iy * dy)) / dy

        g00 = self.grid[ix, iy, :].dot([1.0 - u, 1.0 - v])
        g10 = self.grid[ix + 1, iy, :].dot([u, 1.0 - v])
        g01 = self.grid[ix, iy + 1, :].dot([1.0 - u, v])
        g11 = self.grid[ix + 1, iy + 1, :].dot([u, v])

        a = interpolate(1.0 - u, g00, g10)
        b = interpolate(1.0 - u, g01, g11)
        c = interpolate(1.0 - v, a, b)

        return c


def fade(t):
    return t*t*t*(t*(t*6.0 - 15.0) + 10.0)


def interpolate(x, a, b):
    x = fade(x)
    return (x * a) + ((1.0 - x) * b)
