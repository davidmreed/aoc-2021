from pathlib import Path
from itertools import tee

data = [int(x) for x in Path("input-day1.txt").read_text().strip().split("\n")]

print(sum(b > a for (a, b) in zip(data, data[1:])))

puzzle2, puzzle3 = tee(zip(data, data[1:], data[2:]), 2)
next(puzzle3)

print(sum(b > a for (a, b) in zip(map(sum, puzzle2), map(sum, puzzle3))))
