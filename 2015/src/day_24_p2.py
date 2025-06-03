import functools
import itertools
import operator

from libs import io


def _can_partition_2(packages: list[int], target_sum: int) -> bool:

    @functools.cache
    def _solve(idx: int, cur_sum: int) -> bool:
        if cur_sum == target_sum:
            return True
        if idx >= len(packages):
            return False

        if _solve(idx + 1, cur_sum):
            return True
        if _solve(idx + 1, cur_sum + packages[idx]):
            return True
        return False

    return _solve(0, 0)


def _can_partition_3(packages: list[int], target_sum: int) -> bool:

    def _get_remaining_packages(mask: int) -> list[int]:
        remaining_packages = []
        for i in range(len(packages)):
            if mask & (1 << i) == 0:
                remaining_packages.append(packages[i])
        return remaining_packages

    @functools.cache
    def _solve(mask: int, cur_sum: int) -> bool:
        if cur_sum == target_sum:
            remaining_packages = _get_remaining_packages(mask)
            return _can_partition_2(remaining_packages, target_sum)

        for i in range(len(packages)):
            if mask & (1 << i) != 0:
                continue
            if _solve(mask | (1 << i), cur_sum + packages[i]):
                return True

        return False

    return _solve(0, 0)


packages_arr = [int(line) for line in io.read_lines()]
assert len(packages_arr) == len(set(packages_arr))
packages = set(packages_arr)

total_sum = sum(packages)
assert total_sum % 4 == 0
target_sum = total_sum // 4

ans = float("inf")

for i in range(1, len(packages)):
    for first_group in itertools.combinations(packages, i):
        if sum(first_group) != target_sum:
            continue
        remaining_packages = packages - set(first_group)
        if _can_partition_3(list(remaining_packages), target_sum):
            ans = min(ans, functools.reduce(operator.mul, first_group))

    if ans != float("inf"):
        break

print(ans)
