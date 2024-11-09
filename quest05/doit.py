
def load_input(file_path):
    with open(file_path, 'r') as file:
        return file.read().strip()

def solve(input):
    columns = []
    for line in input.split("\n"):
        values = line.split(" ")
        for i, value in enumerate(values):
            if len(columns) <= i:
                columns.append([])
            columns[i].append(int(value))

    clap_idx = 0
    for _ in range(10):
        clapper = columns[clap_idx].pop(0)
        target_column = columns[(clap_idx + 1) % 4]
        moves = abs((clapper % (len(target_column) * 2)) - 1)
        if moves > len(target_column):
            moves = (len(target_column) * 2) - moves
        target_column.insert(moves, clapper)
        clap_idx = (clap_idx + 1) % len(columns)

    answer = "".join(str(column[0]) for column in columns)
    return answer

def solve_part_ii(input):
    columns = []
    for line in input.split("\n"):
        values = line.split(" ")
        for i, value in enumerate(values):
            if len(columns) <= i:
                columns.append([])
            columns[i].append(int(value))
    output = open("py_output.txt", "w")

    count = {}
    r = 1
    clap_idx=0
    while True:
        clapper = columns[clap_idx].pop(0)
        target_column = columns[(clap_idx + 1) % 4]
        moves = abs((clapper % (len(target_column) * 2)) - 1)
        if moves > len(target_column):
            moves = (len(target_column) * 2) - moves
        target_column.insert(moves, clapper)
        clap_idx = (clap_idx + 1) % len(columns)
        result = "".join(str(column[0]) for column in columns)
        count[result] = count.get(result, 0) + 1
        output.write(f"{r} {result}\n")
        if count[result] == 2024:
            print(r, int(result))
            answer = r * int(result)
            return answer
        r += 1

def solve_part_iii(input):
    columns = []
    for line in input.split("\n"):
        values = line.split(" ")
        for i, value in enumerate(values):
            if len(columns) <= i:
                columns.append([])
            columns[i].append(int(value))

    cache = {}
    answer = float('-inf')
    r = 1
    while True:
        state = "|".join("".join(str(x) for x in column) for column in columns)
        if state in cache:
            break
        cache[state] = True
        clapper = columns[clap_idx].pop(0)
        target_column = columns[(clap_idx + 1) % 4]
        moves = abs((clapper % (len(target_column) * 2)) - 1)
        if moves > len(target_column):
            moves = (len(target_column) * 2) - moves
        target_column.insert(moves, clapper)
        clap_idx = (clap_idx + 1) % len(columns)
        top = "".join(str(column[0]) for column in columns)
        answer = max(answer, int(top))
        r += 1

    return answer

if __name__ == "__main__":
    input_part1 = load_input('src/part1.txt')
    input_part2 = load_input('src/part2.txt')
    # input_part3 = load_input('src/part3.txt')

    print("Part I Answer:", solve(input_part1))
    print("Part II Answer:", solve_part_ii(input_part2))
    # print("Part III Answer:", solve_part_iii(input_part3))
