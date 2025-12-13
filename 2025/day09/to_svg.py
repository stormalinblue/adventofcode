def scale(coord):
    return coord * 500 / 100000


with open("input09.txt") as f:
    points = [[int(x) for x in line.strip().split(",")] for line in f]

    point_strs = []
    for point in points:
        point_strs.append(f"{point[0] * 500 / 100000},{point[1] * 500 / 100000}")

    point_fill = " ".join(point_strs)

    print('<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">')
    print(f'<polygon points="{point_fill}" fill="lightblue" stroke="black" />')

    for point in points:
        print(
            f'<circle cx="{point[0] * 500 / 100000}" cy="{point[1] * 500 / 100000}" fill="black" stroke="none" r="2"/>'
        )

    first_point = (1730, 50238)
    second_point = (94710, 50238)
    y = scale(min(first_point[1], second_point[1]))
    x = scale(min(first_point[0], second_point[0]))
    width = scale(abs(first_point[0] - second_point[0]))
    height = scale(abs(first_point[1] - second_point[1]))
    print(
        f'<rect y="{y}" x="{x}" width="{width}" height="{height}" fill="rgba(250, 100, 100, 0.5)"/>'
    )
    print("</svg>")
