import json

from libs import io


def _dfs(root: list | dict | int | str) -> int:
    match root:
        case list():
            return sum(_dfs(item) for item in root)
        case dict():
            return sum(_dfs(item) for item in root.values())
        case int():
            return root
        case _:
            return 0


input = json.loads(io.read_all())
ans = _dfs(input)
print(ans)
