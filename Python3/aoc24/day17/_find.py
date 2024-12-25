"""Module containing computation of solutions."""

from functools import cache

@cache
def run_program_loop(a: int, b: int | None = None, c: int | None = None) -> tuple[int, int, int]:
    """Run a single loop of the input program.

    Note that only the A value is required as an input.
    That is because B and C registers are overwritten each loop.

    Dynamic program implementation was already done in Rust.
    So here my program is hard-coded.

        B = A &  7
        B = B ^  3
        C = A >> B
        B = B ^  C
        B = B ^  3
        <print B>
        A = A >> 3

    Parameters
    ----------
    a : int
        Value of the A register.

    Returns
    -------
    (int, int, int)
        The values of the A,B,C registers after a loop.

    """
    b: int = b or 0
    c: int = c or 0

    # Program
    b = a % 8
    b = b ^ 3
    c = a >> b
    b = b ^ c
    b = b ^ 3
    # a = a >> 3

    # Test program
    # b = a % 8

    return (a, b, c)


def run_program_until_exit(a: int, b: int | None = None, c: int | None = None) -> list[int]:
    """Run the program until it exists.

    Parameters
    ----------
    a : int
        Initial value of the A register.

    Returns
    -------
    list[int]
        Values outputted by the program.

    """
    result: list[int] = []

    while a != 0:
        a = a >> 3
        a, b, c = run_program_loop(a, b, c)
        result.append(b%8)

    # print("Running exited with", a, b, c)
    return result


@cache
def find_next_bits_of_a(target_results: tuple[int], current_a: int) -> int | None:
    """Brute force the next few bits of A."""
    # Recursion limit
    if len(target_results) == 0:
        return current_a

    # Brute-forcing the last 3 bits

    shifted_a: int = current_a << 3

    for add_bytes in range(8):
        (a, b, _c) = run_program_loop(shifted_a | add_bytes)
        assert a == shifted_a | add_bytes

        # Since the program prints B
        if b % 8 != target_results[-1]:
            continue

        # Call recursively, test if these bytes will work later on
        next_res: int | None = find_next_bits_of_a(target_results[:-1], a)
        if next_res is None:
            continue

        return next_res

    return None
