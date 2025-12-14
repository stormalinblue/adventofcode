import math
from itertools import combinations, product
from typing import NamedTuple

import numpy as np
import sympy as sp

ProblemRow = NamedTuple(
    "ProblemRow", [("switches", list[list[int]]), ("target_joltage", list[int])]
)


def prod(iter):
    result = 1
    for x in iter:
        result *= x
    return result


def parse_input(filename) -> list[ProblemRow]:
    result: list[ProblemRow] = []
    with open(filename) as in_file:
        for line in in_file:
            parts = line.split()
            joltage_str = parts[-1]
            switches_str = parts[1:-1]

            switches = [[int(x) for x in sw[1:-1].split(",")] for sw in switches_str]

            joltages = [int(x) for x in joltage_str[1:-1].split(",")]

            result.append(ProblemRow(switches=switches, target_joltage=joltages))
    return result


if __name__ == "__main__":
    import sys

    filename = sys.argv[1]
    data = parse_input(filename)

    total_switches = 0

    for problem_number, row in enumerate(data):
        print("PROBLEM: ", problem_number)

        target = np.asarray(row.target_joltage)

        basis = []
        for switch in row.switches:
            switch_vec = np.zeros(target.shape, np.int32)
            switch_vec[switch] = 1

            basis.append(switch_vec)

        basis = np.stack(basis, axis=1)
        # print(target)
        # print("basis", basis)
        num_switches = len(row.switches)

        augmented = np.concatenate([basis, target[:, np.newaxis]], axis=1)

        a = sp.Matrix(basis)
        b = sp.Matrix(target)
        rref_solution = a.gauss_jordan_solve(b)
        x_space = rref_solution[0]
        params = rref_solution[1]

        print(x_space)

        if len(params) == 0:
            solution = np.asarray(x_space).astype(np.int32)
            print("exact solution", solution)
            total_switches += sum(solution)
        else:
            best_solution = None
            best_solution_switches = float("inf")
            num_params = len(params)

            par_maxes = {}
            par_mins = {}
            for param in params:
                par_mins[param] = float("inf")
                par_maxes[param] = -float("inf")

            for vertex_expressions in combinations(x_space, num_params):
                # print("expressions", vertex_expressions)
                result = sp.solve([sp.Eq(exp, 0) for exp in vertex_expressions])

                if result == [] or len(result) < num_params:
                    continue

                if not all(len(v.free_symbols) == 0 for v in result.values()):
                    continue

                if not all(cell.subs(result) >= 0 for cell in x_space):
                    continue

                print("vertex", result)
                for param, value in result.items():
                    par_maxes[param] = max(par_maxes[param], value.evalf())
                    par_mins[param] = min(par_mins[param], value.evalf())

            # for cell in x_space:
            #     if len(cell.free_symbols) == 1:
            #         symbol = next(iter(cell.free_symbols))
            #         decreasing = cell.subs(symbol, 0) > cell.subs(symbol, 1)

            #         if decreasing:
            #             bound = sp.solve(sp.Eq(cell, 0), symbol)[0]
            #             par_maxes[symbol] = min(par_maxes[symbol], bound.evalf())

            # print("maxes", par_maxes)

            par_range_caps = {
                param: int(math.ceil(par_maxes[param] + 1)) for param in params
            }

            print(
                "num params", num_params, "search space", prod(par_range_caps.values())
            )
            for values in product(*[range(par_range_caps[param]) for param in params]):
                particular = x_space.subs(dict(zip(params, values)))
                particular = np.asarray(particular).astype(np.float32)

                if np.any(particular < 0):
                    continue
                elif np.any(particular != np.round(particular)):
                    continue
                else:
                    sw = np.sum(particular)
                    if sw < best_solution_switches:
                        best_solution = particular
                        best_solution_switches = sw

            solution = np.array(best_solution).flatten().astype(np.int32)
            print("found solution", solution, target)
            assert np.all(basis @ solution == target)

            total_switches += sum(solution)

    print("Total switches", total_switches)
    # break
