import functools
from typing import Literal

from pydantic import BaseModel

from libs import io


class _UnaryOp(BaseModel):
    kind: Literal["NOT", "IDENTITY"]
    inp: str | int
    out: str


class _BinaryOp(BaseModel):
    kind: Literal["AND", "OR", "LSHIFT", "RSHIFT"]
    inp1: str | int
    inp2: str | int
    out: str


_Op = _UnaryOp | _BinaryOp


def _try_int(x: str) -> str | int:
    try:
        return int(x)
    except ValueError:
        return x


def _parse_line(line: str) -> _Op:
    tokens = line.split()

    match tokens:
        case [val, "->", out]:
            return _UnaryOp(kind="IDENTITY", inp=_try_int(val), out=out)
        case ["NOT", val, "->", out]:
            return _UnaryOp(kind="NOT", inp=_try_int(val), out=out)
        case [val1, op_kind, val2, "->", out]:
            return _BinaryOp(
                kind=op_kind,  # type: ignore
                inp1=_try_int(val1),
                inp2=_try_int(val2),
                out=out,
            )
        case _:
            assert False, tokens


input = [_parse_line(line) for line in io.read_lines()]
op_map: dict[str, _Op] = {op.out: op for op in input}


@functools.cache
def _get_val(inp: str | int) -> int:
    if isinstance(inp, int):
        return inp

    op = op_map[inp]
    match op:
        case _UnaryOp(kind="IDENTITY"):
            return _get_val(op.inp)
        case _UnaryOp(kind="NOT"):
            return ~_get_val(op.inp)
        case _BinaryOp(kind="AND"):
            return _get_val(op.inp1) & _get_val(op.inp2)
        case _BinaryOp(kind="OR"):
            return _get_val(op.inp1) | _get_val(op.inp2)
        case _BinaryOp(kind="LSHIFT"):
            return _get_val(op.inp1) << _get_val(op.inp2)
        case _BinaryOp(kind="RSHIFT"):
            return _get_val(op.inp1) >> _get_val(op.inp2)
        case _:
            assert False


print(_get_val("a"))
