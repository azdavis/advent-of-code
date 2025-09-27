import re
from dataclasses import dataclass
from typing import Generator, Union

RE = re.compile(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")

def assert_str(x: object) -> str:
    if isinstance(x, str):
        return x
    raise TypeError(f"expected str, found {type(x)}: {x}")

@dataclass
class Do:
    pass

@dataclass
class Dont:
    pass

@dataclass
class Mul:
    lhs: int
    rhs: int

Action = Union[Do, Dont, Mul]

def parse() -> Generator[Action, None, None]:
    with open("input/03.txt") as f:
        gen = (assert_str(x) for line in f.read().splitlines() for x in RE.findall(line))
    for s in gen:
        if s == "do()":
            yield Do()
        elif s == "don't()":
            yield Dont()
        elif s.startswith("mul("):
            s = s.removeprefix("mul(").removesuffix(")")
            a, b = s.split(",")
            yield Mul(int(a), int(b))

def p1() -> int:
    return sum(x.lhs * x.rhs for x in parse() if isinstance(x, Mul))

def p2() -> int:
    on = True
    ret = 0
    for x in parse():
        if isinstance(x, Do):
            on = True
        elif isinstance(x, Dont):
            on = False
        elif isinstance(x, Mul) and on:
            ret += x.lhs * x.rhs
    return ret

print(p2())
