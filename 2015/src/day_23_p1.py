from libs import io

instructions = io.read_lines()
reg = {"a": 0, "b": 0}
idx = 0

while True:
    if not 0 <= idx < len(instructions):
        break

    instruction = instructions[idx]
    found_offset: str | None = None

    match instruction.split():
        case ["hlf", r]:
            reg[r] //= 2
        case ["tpl", r]:
            reg[r] *= 3
        case ["inc", r]:
            reg[r] += 1
        case ["jmp", offset]:
            found_offset = offset
        case ["jie", r, offset]:
            if reg[r.rstrip(",")] % 2 == 0:
                found_offset = offset
        case ["jio", r, offset]:
            if reg[r.rstrip(",")] == 1:
                found_offset = offset

    if not found_offset:
        idx += 1
        continue

    val = int(found_offset[1:])
    if found_offset.startswith("+"):
        idx += val
    else:
        idx -= val


print(reg["b"])
