def fibonacci_recursive(n: int) -> int:
    if n <= 1:
        return 1
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def fibonacci_iterative(n: int) -> int:
    if n <= 1:
        return 1
    a, b = 1, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def build_dict(size: int) -> dict:
    return {i: i * 2 for i in range(size)}


def test_fib_recursive_20(codspeed_benchmark):
    codspeed_benchmark(fibonacci_recursive, 20)


def test_fib_iterative_20(codspeed_benchmark):
    codspeed_benchmark(fibonacci_iterative, 20)


def test_fib_recursive_25(codspeed_benchmark):
    codspeed_benchmark(fibonacci_recursive, 25)


def test_fib_iterative_25(codspeed_benchmark):
    codspeed_benchmark(fibonacci_iterative, 25)


def test_build_dict_100(codspeed_benchmark):
    codspeed_benchmark(build_dict, 100)


def test_build_dict_1000(codspeed_benchmark):
    codspeed_benchmark(build_dict, 1000)


def test_build_dict_10000(codspeed_benchmark):
    codspeed_benchmark(build_dict, 10000)
