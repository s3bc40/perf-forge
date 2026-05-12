import random

RAND = random.Random(42)


def sum_of_squares(n: int) -> int:
    total = 0
    for i in range(n):
        total += i * i
    return total


def sort_random_list(n: int) -> list:
    items = [RAND.random() for _ in range(n)]
    items.sort()
    return items


def test_sum_of_squares(codspeed_benchmark):
    codspeed_benchmark(sum_of_squares, 100_000)


def test_sort_random_list(codspeed_benchmark):
    codspeed_benchmark(sort_random_list, 10_000)


def test_dict_lookup(codspeed_benchmark):
    n = 10_000
    d = {k: k * 2 for k in range(n)}

    def bench():
        for k in range(n):
            _ = d[k]

    codspeed_benchmark(bench)
