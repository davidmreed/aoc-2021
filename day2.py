from pathlib import Path

data = Path("day2.txt").read_text().strip().split("\n")

position = [0, 0, 0]
for line in data:
    (command, distance) = line.strip().split(" ")
    if command == "forward":
        position[0] += int(distance)
        position[1] += position[2] * int(distance)
    elif command == "up":
        position[2] -= int(distance)
    elif command == "down":
        position[2] += int(distance)

print(f"I have {position[0] * position[1]}")
