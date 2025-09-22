from collections import Counter

def parse() -> tuple[list[int], list[int]]:
    with open("input/01.txt") as f:
        input = f.read()
    fst: list[int] = []
    snd: list[int] = []
    for line in input.splitlines():
        a, b = line.strip().split()
        fst.append(int(a))
        snd.append(int(b))
    return fst, snd

def p1():
    fst, snd = parse()
    fst.sort()
    snd.sort()
    return sum(abs(a - b) for a, b in zip(fst, snd))

def p2():
    fst, snd = parse()
    c = Counter(snd)
    ret = 0
    for x in fst:
        ret += c[x] * x
    return ret

print(p2())
