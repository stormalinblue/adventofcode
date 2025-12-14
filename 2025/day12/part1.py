from typing import NamedTuple, cast

ProblemPair = NamedTuple(
    "ProblemPair", [("dims", tuple[int, int]), ("amounts", list[int])]
)
ProblemData = NamedTuple(
    "ProblemData", [("tiles", list[list[str]]), ("problems", list[ProblemPair])]
)


def parse_problem_line(line) -> ProblemPair:
    dims_str, _, nums_str = line.partition(": ")
    print(line)
    dims = cast(tuple[int, int], tuple(int(x) for x in dims_str.split("x")))
    nums = list(int(x) for x in nums_str.split())
    return ProblemPair(dims, nums)


def parse_input(filename):
    tiles = []
    problems = []
    with open(filename) as in_file:
        while True:
            line = in_file.readline().strip()
            print("found line", repr(line))
            tile = []
            if ":" in line and "x" not in line:
                while True:
                    line = in_file.readline().strip()
                    print("tile found line", repr(line))
                    tile.append(line)
                    if line == "":
                        break
            else:
                break
            tiles.append(tile)

        problems = [parse_problem_line(line)]
        for line in in_file:
            problems.append(parse_problem_line(line))

    return ProblemData(tiles, problems)


if __name__ == "__main__":
    import sys

    filename = sys.argv[1]

    data = parse_input(filename)

    tiles = data.tiles
    tile_areas = []

    for tile in tiles:
        tile_areas.append(sum(line.count("#") for line in tile))

    tot_can_fit: int = 0
    for problem in data.problems:
        dims = problem.dims
        total_area = dims[0] * dims[1]

        if total_area < sum(
            tile_areas[i] * problem.amounts[i] for i in range(len(tiles))
        ):
            continue
        else:
            side_by_side_x = dims[0] // 3
            side_by_side_y = dims[1] // 3

            if side_by_side_x * side_by_side_y >= sum(problem.amounts):
                tot_can_fit += 1

    print("total can fit", tot_can_fit)
