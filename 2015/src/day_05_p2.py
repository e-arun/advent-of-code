from collections import Counter

from libs import io


def _is_nice(xs: str) -> bool:
    ctr: Counter[str] = Counter()
    # Count the first pair since it won't be counted in the subsequent loop
    ctr[xs[:2]] += 1

    satisfies_rule_2 = False

    for a, b, c in zip(xs, xs[1:], xs[2:]):
        if not a == b == c:
            ctr[b + c] += 1
        if a == c:
            satisfies_rule_2 = True

    most_freq_pair = ctr.most_common(1)[0][1]
    return satisfies_rule_2 and most_freq_pair > 1


input = io.read_lines()
ans = sum(_is_nice(x) for x in input)
print(ans)
