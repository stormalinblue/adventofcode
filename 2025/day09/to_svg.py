with open("../test.txt") as f:
    points = [[int(x) for x in line.strip().split(",")] for line in f]

    first_point = (11, 1)
    second_point = (9, 7)
    max_extent = max(
        max(max(point) for point in points), max(first_point), max(second_point)
    )

    def scale(coord):
        return coord * 500 / max_extent

    point_strs = []
    for point in points:
        point_strs.append(f"{scale(point[0])},{scale(point[1])}")

    point_fill = " ".join(point_strs)

    print('<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">')
    print(f'<polygon points="{point_fill}" fill="lightblue" stroke="black" />')

    for point in points:
        print(
            f'<circle cx="{scale(point[0])}" cy="{scale(point[1])}" fill="black" stroke="none" r="2"/>'
        )

    y = scale(min(first_point[1], second_point[1]))
    x = scale(min(first_point[0], second_point[0]))
    width = scale(abs(first_point[0] - second_point[0]))
    height = scale(abs(first_point[1] - second_point[1]))
    print(
        f'<rect y="{y}" x="{x}" width="{width}" height="{height}" fill="rgba(250, 100, 100, 0.5)"/>'
    )
    print("</svg>")
