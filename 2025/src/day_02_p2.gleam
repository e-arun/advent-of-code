import gleam/function
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder
import stdin

fn all_same(xs: List(List(item))) -> Bool {
  case xs {
    [first, _, ..] as arr -> list.all(arr, fn(x) { x == first })
    _ -> False
  }
}

fn is_valid(xs: Int) -> Bool {
  let xs = int.to_string(xs) |> string.to_graphemes
  let n = list.length(xs)

  yielder.range(1, n / 2)
  |> yielder.map(list.sized_chunk(xs, _))
  |> yielder.map(all_same)
  |> yielder.any(function.identity)
}

fn parse_segment(s: String) -> #(Int, Int) {
  let assert [a, b] =
    s |> string.split("-") |> list.map(int.parse) |> result.values
  #(a, b)
}

pub fn main() {
  stdin.read_lines()
  |> yielder.to_list
  |> list.flat_map(string.split(_, ","))
  |> list.map(string.trim)
  |> list.map(parse_segment)
  |> list.flat_map(fn(x) { list.range(x.0, x.1) })
  |> list.filter(is_valid)
  |> int.sum
  |> int.to_string
  |> io.println
}
