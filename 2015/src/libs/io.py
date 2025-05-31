import sys


def read_all() -> str:
    return sys.stdin.read().strip()


def read_lines() -> list[str]:
    return [line.strip() for line in sys.stdin.readlines() if line.strip()]
