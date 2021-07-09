import math
import numpy as np
import random

from .smooth import interpolate, smoothstep_1


inv_sqrt_2 = 1.0 / math.sqrt(2.0)


class Perlin:
    """
    Perlin noise map.
    """

    def __init__(self, res, smoothstep):
        """
        Construct a new instance,
        with a given resolution of gradient vectors.
        """
        assert(res[0] >= 2)
        assert(res[1] >= 2)

        self.smoothstep = smoothstep

        self.gradients = np.empty((res[0], res[1], 3))

        for iy in range(res[1]):
            for ix in range(res[0]):
                theta = random.uniform(0.0, 2.0 * math.pi)
                self.gradients[ix, iy, 0] = math.sin(theta)
                self.gradients[ix, iy, 1] = math.cos(theta)
                self.gradients[ix, iy, 2] = random.uniform(0.0, 1.0)

    def sample(self, pos):
        """
        Sample the noise map.
        Return a value in the range [-1:1]
        """
        (nx, ny, _) = self.gradients.shape

        # Move point inside the gradients.
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
        g00 = self.gradients[ix0, iy0, 0:2].dot([u, v])
        g10 = self.gradients[ix1, iy0, 0:2].dot([1.0 - u, v])
        g01 = self.gradients[ix0, iy1, 0:2].dot([u, 1.0 - v])
        g11 = self.gradients[ix1, iy1, 0:2].dot([1.0 - u, 1.0 - v])

        # Interpolate
        a = interpolate(u, g10, g00, self.smoothstep)
        b = interpolate(u, g11, g01, self.smoothstep)
        c = interpolate(v, b, a, self.smoothstep)

        return c * inv_sqrt_2

    def rotate(self, dt):
        """
        Rotate the gradient vectors.
        """
        (nx, ny, _) = self.gradients.shape

        for xi in range(nx):
            for yi in range(ny):
                x = self.gradients[xi, yi, 0]
                y = self.gradients[xi, yi, 1]
                r = self.gradients[xi, yi, 2] * dt

                cs = math.cos(r)
                sn = math.sin(r)

                self.gradients[xi, yi, 0] = (x * cs) - (y * sn)
                self.gradients[xi, yi, 1] = (x * sn) + (y * cs)
