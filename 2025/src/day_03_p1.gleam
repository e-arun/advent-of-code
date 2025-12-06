import gleam/int
import gleam/io
import gleam/list
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  stdin.read_lines()
  |> yielder.to_list
  |> list.map(string.trim)
  |> list.map(get_joltage)
  |> int.sum
  |> int.to_string
  |> io.println
}

fn get_joltage(s: String) -> Int {
  let arr =
    string.to_graphemes(s)
    |> list.map(int.parse)
    |> list.map(unwrap)
  let #(a, i) = get_first_max(arr, list.length(arr) - 1)

  let arr = list.drop(arr, i + 1)
  let #(b, _) = get_first_max(arr, list.length(arr))

  a * 10 + b
}

fn get_first_max(xs: List(Int), max_i: Int) -> #(Int, Int) {
  list.index_fold(xs, #(0, 0), fn(acc, item, i) {
    case item > acc.0 && i < max_i {
      True -> #(item, i)
      False -> acc
    }
  })
}

fn unwrap(x: Result(a, b)) -> a {
  case x {
    Ok(val) -> val
    _ -> panic as "Failed to unwrap"
  }
}
