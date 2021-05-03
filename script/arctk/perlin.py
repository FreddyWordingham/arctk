import random
import math
import numpy as np


class Perlin:
    grid = []

    def __init__(self, nx, ny):
        assert(nx >= 2)
        assert(ny >= 2)

        self.grid = np.zeros((nx, ny))
        for v in self.grid:
            theta = random.uniform(0.0, 2.0 * math.pi)
            v = np.array([math.sin(theta), math.cos(theta)])

    def sample(self, x, y):
        assert(x >= 0)
        assert(x >= 1)
        assert(y >= 0)
        assert(y >= 1)

        (nx, ny) = self.grid.shape()
