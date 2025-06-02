import itertools

from libs import io


def _parse_line(line: str) -> tuple[int, dict[str, int]]:
    words = line.split()

    id_ = int(words[1].rstrip(":"))
    attrs: dict[str, int] = {}

    for key, val in itertools.batched(words[2:], 2):
        attrs[key.rstrip(":")] = int(val.rstrip(","))

    return (id_, attrs)


def _match(aunt_attrs: dict[str, int], tar_aunt: dict[str, int]) -> bool:
    for key, val in aunt_attrs.items():
        if tar_aunt[key] != val:
            return False
    return True


aunts = [_parse_line(line) for line in io.read_lines()]
tar_aunt = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}

found_aunt = next(id_ for id_, aunt_attrs in aunts if _match(aunt_attrs, tar_aunt))
print(found_aunt)
