from typing import cast


def parse_input(filename) -> list[tuple[int, int]]:
    with open(filename) as f:
        line = f.readline().strip()

        return [
            cast(tuple[int, int], tuple(int(x) for x in pair.split("-")))
            for pair in line.split(",")
        ]


def process_pair(pair: tuple[int, int]) -> int:
    start, end = pair

    result = 0
    for i in range(start, end + 1):
        # print(i)
        num_str = str(i)
        str_len = len(num_str)
        for parts in range(2, 2 + 1):
            if str_len % parts == 0:
                part_len = str_len // parts
                first_part = num_str[:part_len]

                if first_part * parts == num_str:
                    print(i, parts)
                    result += i
                    break

    # print("pair", start, end, result)
    return result


if __name__ == "__main__":
    import sys

    filename = sys.argv[1]

    pairs = parse_input(filename)
    print(sum(process_pair(pair) for pair in pairs))
