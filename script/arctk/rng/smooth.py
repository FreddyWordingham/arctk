
def smoothstep_1(t):
    "Linear (no) smoothing"
    return t


def smoothstep_2(t):
    "Quadratic smoothing"
    return (3.0 * t**2) - (2.0 * t**3)


def smoothstep_3(t):
    "Cubic smoothing"
    return (10.0 * t**3) - (15.0 * t**4) + (6.0 * t**5)


def smoothstep_4(t):
    "Fourth degree smoothing"
    return (35.0 * t**4) - (84.0 * t**5) + (70.0 * t**6) - (20.0 * t**7)


def smoothstep_5(t):
    "Fifth degree smoothing"
    return (126.0 * t**5) - (420.0 * t**6) + (540.0 * t**7) - (315.0 * t**8) + (70.0 * t**9)


def smoothstep_6(t):
    "Sitth degree smoothing"
    return (462.0 * t**6) - (1980.0 * t**7) + (3465.0 * t**8) - (3080.0 * t**9) + (1386.0 * t**10) - (252.0 * t**11)


def smoothstep_7(t):
    "Seventh degree smoothing"
    return (1716.0 * t**7) - (9009.0 * t**8) + (20020.0 * t**9) - (24024.0 * t**10) + (16380.0 * t**11) - (6006.0 * t**12) + (924.0 * t**13)


def interpolate(t, a, b, ss):
    "Interpolation function"
    return a + (ss(t) * (b - a))
