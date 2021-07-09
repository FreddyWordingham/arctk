from .smooth import interpolate, smoothstep_1
from .perlin import Perlin


class PerlinStack:
    """
    Multiple resolution Perlin noise map.
    """

    def __init__(self, resolutions, smoothstep):
        """
        Construct a new instance,
        with a given resolution of gradient vectors.
        """

        nz = len(resolutions)
        self.maps = []
        for res in resolutions:
            self.maps.append(Perlin(res, smoothstep))

    def sample(self, pos):
        """
        Sample the noise map.
        Return a value in the range [-1:1]
        """

        sum = 0.0
        for map in self.maps:
            sum += map.sample(pos)

        return sum / len(self.maps)

    def rotate(self, dt):
        """
        Rotate the gradient vectors.
        """

        for map in self.maps:
            map.rotate(dt)
