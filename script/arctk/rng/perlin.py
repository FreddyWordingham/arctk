import math
import numpy as np
import random

from .smooth import interpolate, smoothstep_1


class Perlin:
    gradients = []
    smoothstep = []

    def __init__(self, res):
        """
        Construct a new instance,
        with a given resolution of gradient vectors.
        """
        assert(res[0] >= 2)
        assert(res[1] >= 2)

        self.smoothstep = smoothstep_1

        self.grid = np.empty((res[0], res[1], 2))

        for iy in range(res[1]):
            for ix in range(res[0]):
                theta = random.uniform(0.0, 2.0 * math.pi)
                self.grid[ix, iy, 0] = math.sin(theta)
                self.grid[ix, iy, 1] = math.cos(theta)

    def sample(self, pos):
        """
        Sample the noise map.
        Return a value in the range [-1:1]
        """
        (nx, ny, _) = self.grid.shape

        # Move point inside the grid.
        x = math.modf(pos[0])[0]
        y = math.modf(pos[1])[0]

        # Indices
        ix0 = math.floor(x * nx)
        ix1 = (ix0 + 1) % nx
        iy0 = math.floor(y * ny)
        iy1 = (iy0 + 1) % ny

        # UV coordinates
        u = 1.0 - (x * nx) + ix0
        v = 1.0 - (y * ny) + iy0

        # Gradient vectors dot offset vectors
        g00 = self.grid[ix0, iy0, 0:2].dot([u, v])
        g10 = self.grid[ix1, iy0, 0:2].dot([1.0 - u, v])
        g01 = self.grid[ix0, iy1, 0:2].dot([u, 1.0 - v])
        g11 = self.grid[ix1, iy1, 0:2].dot([1.0 - u, 1.0 - v])

        # Interpolate
        a = interpolate(u, g10, g00, self.smoothstep)
        b = interpolate(u, g11, g01, self.smoothstep)
        c = interpolate(v, b, a, self.smoothstep)

        return c
