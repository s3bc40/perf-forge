def list_grow(n: int) -> list:
    v = []
    for i in range(n):
        v.append(i)
    return v


def list_grow_with_reserve(n: int) -> list:
    v = [0] * n
    for i in range(n):
        v[i] = i
    return v


def test_list_grow_100(codspeed_benchmark):
    codspeed_benchmark(list_grow, 100)


def test_list_grow_1000(codspeed_benchmark):
    codspeed_benchmark(list_grow, 1000)


def test_list_grow_10000(codspeed_benchmark):
    codspeed_benchmark(list_grow, 10000)


def test_list_grow_with_reserve_100(codspeed_benchmark):
    codspeed_benchmark(list_grow_with_reserve, 100)


def test_list_grow_with_reserve_1000(codspeed_benchmark):
    codspeed_benchmark(list_grow_with_reserve, 1000)


def test_list_grow_with_reserve_10000(codspeed_benchmark):
    codspeed_benchmark(list_grow_with_reserve, 10000)


def test_many_small_strings(codspeed_benchmark):
    def bench():
        return [str(i) for i in range(1000)]

    codspeed_benchmark(bench)


def test_few_large_strings(codspeed_benchmark):
    def bench():
        return "".join(str(i) for i in range(1000))

    codspeed_benchmark(bench)
