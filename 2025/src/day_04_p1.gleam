import gleam/bool
import gleam/dict.{type Dict}
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
  |> list.map(string.trim)
  |> build_mat
  |> count_ans
  |> int.to_string
  |> io.println
}

type Mat =
  Dict(#(Int, Int), String)

fn build_mat(lines: List(String)) -> Mat {
  list.index_map(lines, fn(line, i) {
    string.to_graphemes(line)
    |> list.index_map(fn(char, j) { #(#(i, j), char) })
  })
  |> list.flatten
  |> dict.from_list
}

fn count_ans(mat: Mat) -> Int {
  dict.to_list(mat)
  |> list.filter(fn(x) { x.1 == "@" })
  |> list.map(fn(x) { count_adj(mat, x.0) })
  |> list.count(fn(x) { x < 4 })
}

fn count_adj(mat: Mat, point: #(Int, Int)) -> Int {
  let #(x, y) = point

  iter_2d([-1, 0, 1], [-1, 0, 1])
  |> list.map(fn(diff) {
    use <- bool.guard(diff == #(0, 0), 0)

    let pos = #(x + diff.0, y + diff.1)
    let val = dict.get(mat, pos) |> result.unwrap(".")

    case val == "@" {
      True -> 1
      False -> 0
    }
  })
  |> int.sum
}

fn iter_2d(xs: List(x), ys: List(y)) -> List(#(x, y)) {
  list.flat_map(xs, fn(x) { list.map(ys, fn(y) { #(x, y) }) })
}
