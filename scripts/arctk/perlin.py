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
