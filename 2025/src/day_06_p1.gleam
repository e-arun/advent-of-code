import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  stdin.read_lines()
  |> yielder.to_list
  |> list.map(parse_line)
  |> list.reverse
  |> list.transpose
  |> list.map(get_ans)
  |> int.sum
  |> int.to_string
  |> io.println
}

fn parse_line(s: String) -> List(String) {
  s |> string.trim |> string.split(" ") |> list.filter(fn(x) { x != "" })
}

fn get_ans(xs: List(String)) -> Int {
  let assert [op, ..rest] = xs
  let rest = list.map(rest, int.parse) |> result.values
  case op {
    "*" -> list.fold(rest, 1, fn(acc, x) { acc * x })
    _ -> list.fold(rest, 0, fn(acc, x) { acc + x })
  }
}
