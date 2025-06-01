from libs import io


def _next_password(cur_password: str) -> str:
    assert len(cur_password) > 1
    last_char = cur_password[-1]

    if last_char != "z":
        next_last_char = chr(ord(last_char) + 1)
        return cur_password[:-1] + next_last_char

    return _next_password(cur_password[:-1]) + "a"


def _is_valid(password: str) -> bool:
    for char in "iol":
        if char in password:
            return False

    pairs = set()
    for a, b in zip(password, password[1:]):
        if a == b:
            pairs.add(a)
    if len(pairs) < 2:
        return False

    for a, b, c in zip(password, password[1:], password[2:]):
        if ord(a) + 1 == ord(b) and ord(b) + 1 == ord(c):
            return True

    return False


input = io.read_all()

ans = _next_password(input)
while not _is_valid(ans):
    ans = _next_password(ans)
print(ans)
