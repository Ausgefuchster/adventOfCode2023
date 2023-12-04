import re
import sys

def is_no_dot_left_and_right(line, idx, length):
    if idx == 0:
        return line[idx + length] != '.'
    elif idx + length == len(line): 
        return line[idx - 1] != '.'
    return line[idx - 1] != '.' or line[idx + length] != '.'

def is_no_dot_in_range(line, start, length):
    return any(c != '.' for c in line[start:start+length])

def part_one(lines):
    total = 0
    for i, line in enumerate(lines):
        for match in re.finditer(r"\d{1,3}", line):
            idx = match.start()
            digit = match.group()
            length = len(digit)

            if is_no_dot_left_and_right(line, idx, length):
                total += int(digit)
                continue

            if i > 0 and is_no_dot_in_range(lines[i - 1], max(0, idx - 1), length + 2):
                total += int(digit)
                continue


            if i < len(lines) - 1 and is_no_dot_in_range(lines[i + 1], max(0, idx - 1), length + 2):
                total += int(digit)
                continue

    print(total)


def get_full_number(lines, row, column):
    number = ""
    left = column
    right = column

    while left >= 0 and lines[row][left].isdigit():
        left -= 1

    while right < len(lines[row]) and lines[row][right].isdigit():
        right += 1

    number = lines[row][left + 1:right]

    return int(number)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python part_one.py <inputfile>")
        sys.exit(1)

    input_file = sys.argv[1]
    with open(input_file, 'r') as file:
        lines = file.read().splitlines()

    part_one(lines)
