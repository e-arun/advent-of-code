import gleam/int
import gleam/io
import gleam/list
import gleam/order
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
  let _ids = list.drop(ids, 1) |> list.map(int.parse) |> result.values

  merge_ranges(ranges)
  |> list.map(fn(range) { range.1 - range.0 + 1 })
  |> int.sum
  |> int.to_string
  |> io.println
}

fn parse_range(line: String) -> #(Int, Int) {
  let assert [a, b] =
    string.split(line, "-") |> list.map(int.parse) |> result.values
  #(a, b)
}

fn merge_ranges(ranges: List(#(Int, Int))) -> List(#(Int, Int)) {
  let ranges =
    list.sort(ranges, fn(a, b) {
      case int.compare(a.0, b.0) {
        order.Eq -> int.compare(-a.1, -b.1)
        x -> x
      }
    })

  list.fold(ranges, [], fn(acc: List(#(Int, Int)), range) {
    case acc {
      [] -> [range]
      [first, ..rest] ->
        case range.0 > first.1 {
          True -> [range, first, ..rest]
          False -> [#(first.0, int.max(first.1, range.1)), ..rest]
        }
    }
  })
}
