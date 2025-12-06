import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder
import stdin

fn is_valid(xs: Int) -> Bool {
  let xs = int.to_string(xs)
  let n = string.length(xs)
  case n % 2 == 0 {
    True -> {
      let a = string.drop_start(xs, n / 2)
      let b = string.drop_end(xs, n / 2)
      a == b
    }
    False -> False
  }
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
