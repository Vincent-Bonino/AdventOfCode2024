"""Module containing computation of solutions."""

from .program import Computer


def run_program_loop(program: list[int], a: int) -> tuple[int, int, int]:
    """Run a single loop of the input program.

    Note that only the A value is required as an input.
    That is because B and C registers are overwritten each loop.

    Parameters
    ----------
    a : int
        Value of the A register.

    Returns
    -------
    (int, int, int)
        The values of the A,B,C registers after a loop.

    """
    computer: Computer = Computer(program, a=a, do_shift=False)
    computer.run_until_jump()
    return computer.registers


def find_next_bits_of_a(program: list[int], target_results: list[int], current_a: int) -> int | None:
    """Brute force the next few bits of A."""
    # Recursion limit
    if len(target_results) == 0:
        return current_a

    # Brute-forcing the last 3 bits

    shifted_a: int = current_a << 3

    for add_bytes in range(8):
        (a, b, _c) = run_program_loop(program, a=(shifted_a | add_bytes))
        assert a == shifted_a | add_bytes, f"{a} != {shifted_a} | {add_bytes} = {shifted_a | add_bytes}"

        # Since the program prints B
        if b % 8 != target_results[-1]:
            continue

        # Call recursively, test if these bytes will work later on
        next_res: int | None = find_next_bits_of_a(program, target_results[:-1], a)
        if next_res is None:
            continue

        return next_res

    return None


def solve_part_two(program: list[int]) -> int:
    """Compute solution of part two."""

    # # Test the program
    # a: int = 51342988
    # program = [0,3,5,4,3,0]

    Computer.do_shift = False

    print("Input program:", program)

    result: int = find_next_bits_of_a(list(program), list(program), 0)  # We know A ends at zero to exit the program

    # Test the program
    print("With gotten result :", Computer(program, a=result).run())
    print("With something else:", Computer(program, a=(result << 3)).run())
    print("Expecting           ", program)

    return result