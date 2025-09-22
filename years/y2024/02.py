from typing import Generator

def safe(xs: list[int]) -> bool:
    if len(xs) < 2:
        return False
    increase = xs[0] < xs[1]
    for i in range(1, len(xs)):
        prev = xs[i - 1]
        cur = xs[i]
        if increase != (prev < cur):
            return False
        if not (1 <= abs(prev - cur) <= 3):
            return False
    return True

def parse() -> list[list[int]]:
    ret: list[list[int]] = []
    with open("input/02.txt") as f:
        for line in f:
            ret.append([int(x) for x in line.strip().split()])
    return ret

def p1():
    return sum(1 for xs in parse() if safe(xs))

def remove_one(xs: list[int]) -> Generator[list[int], None, None]:
    for i in range(len(xs)):
        yield [x for j, x in enumerate(xs) if i != j]

def p2():
    return sum(1 for xs in parse() if safe(xs) or any(safe(ys) for ys in remove_one(xs)))

print(p2())
