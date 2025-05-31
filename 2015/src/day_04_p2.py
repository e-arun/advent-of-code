import hashlib

from libs import io

input = io.read_all()
ans = 1

while True:
    key = input + str(ans)
    hash = hashlib.md5(key.encode())
    if hash.hexdigest().startswith("0" * 6):
        break
    ans += 1

print(ans)
