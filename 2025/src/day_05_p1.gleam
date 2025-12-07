import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  let #(ranges, ids) =
    stdin.read_lines()
    |> yielder.to_list
    |> list.map(string.trim)
    |> list.split_while(fn(line) { line != "" })

  let ranges = list.map(ranges, parse_range)
  let ids = list.drop(ids, 1) |> list.map(int.parse) |> result.values

  list.count(ids, is_fresh(_, ranges))
  |> int.to_string
  |> io.println
}

fn parse_range(line: String) -> #(Int, Int) {
  let assert [a, b] =
    string.split(line, "-") |> list.map(int.parse) |> result.values
  #(a, b)
}

fn is_fresh(id: Int, ranges: List(#(Int, Int))) -> Bool {
  list.any(ranges, fn(range) { id >= range.0 && id <= range.1 })
}
