from collections import Counter

from libs import io

input = io.read_all()
ctr = Counter(input)
ans = ctr["("] - ctr[")"]
print(ans)
