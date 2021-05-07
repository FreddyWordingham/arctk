import random
import math
import numpy as np


class Perlin:
    grid = []
    rates = []

    def __init__(self, nx, ny):
        assert(nx >= 2)
        assert(ny >= 2)

        self.grid = np.empty((nx, ny, 2))
        self.rates = np.empty((nx, ny))

        for iy in range(ny):
            t = 0
            for ix in range(nx):
                theta = random.uniform(0.0, 2.0 * math.pi)
                # t += (2.0 * math.pi) / (nx - 1)
                # theta = t
                self.grid[ix, iy, 0] = math.sin(theta)
                self.grid[ix, iy, 1] = math.cos(theta)
                self.rates[ix, iy] = random.uniform(0.0, 1.0)
                # self.rates[ix, iy] = (iy / (ny - 1))

    def update(self, t):
        (nx, ny, _) = self.grid.shape
        for iy in range(ny):
            for ix in range(nx):
                x = self.grid[ix, iy, 0]
                y = self.grid[ix, iy, 1]
                r = self.rates[ix, iy]
                self.grid[ix, iy, 0] = (
                    x * math.cos(t * r)) - (y * math.sin(t * r))
                self.grid[ix, iy, 1] = (
                    x * math.sin(t * r)) + (y * math.cos(t * r))

    def sample(self, x, y):
        assert(x >= 0)
        assert(x <= 1)
        assert(y >= 0)
        assert(y <= 1)
