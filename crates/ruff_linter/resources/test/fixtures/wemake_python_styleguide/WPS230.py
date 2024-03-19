from dataclasses import dataclass


class Violation:
    x1: float
    x2: float
    x3: float
    x4: float
    x5: float
    x6: float
    x7 = "hello"


class AlsoViolation:
    x1: float
    x2: float
    x3: float
    x4: float
    x5: float

    def __init__(self, a, b):
        self.x6: int = a
        self.x7 = b


@dataclass
class Ok:
    x1: float
    x2: float
    x3: float
    x4: float
    x5: float
    x6: float
    x7: float


class AlsoOk:
    x1: float
    x2: float
    x3: float
    x4: float
    x5: float
    x6: object

    def __init__(self):
        self.x6 = object()
        self.x6.nested = "hello"

    @property
    def x7(self):
        return 2.7


class PrivateSoOk:
    _x1: float
    _x2: float
    _x3: float
    _x4: float
    _x5: float
    x6 = "hello"

    def __init__(self):
        self._x7: int = 1234
        self._x8 = 123
