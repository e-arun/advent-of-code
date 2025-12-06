import gleam/int
import gleam/io
import gleam/list
import gleam/pair
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

  list.range(11, 0)
  |> list.map_fold(arr, fn(acc, i) {
    let #(a, a_i) = get_first_max(acc, list.length(acc) - i)
    let acc = list.drop(acc, a_i + 1)
    #(acc, a)
  })
  |> pair.second
  |> list.fold(0, fn(acc, x) { acc * 10 + x })
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
