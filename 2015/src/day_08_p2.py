import json

from libs import io

input = io.read_lines()
ans = sum(len(json.dumps(x)) for x in input)
ans -= sum(len(x) for x in input)
print(ans)
