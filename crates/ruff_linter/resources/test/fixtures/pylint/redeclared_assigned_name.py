FIRST, FIRST = (1, 2)  # PLW0128
FIRST, (FIRST, SECOND) = (1, (1, 2))  # PLW0128
FIRST, (FIRST, SECOND, (THIRD, FIRST)) = (1, (1, 2))  # PLW0128
FIRST, SECOND, THIRD, FIRST, SECOND = (1, 2, 3, 4)  # PLW0128

FIRST, SECOND, _, _, _ignored = (1, 2, 3, 4, 5)  # OK
