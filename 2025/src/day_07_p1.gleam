import gleam/bool
import gleam/dict.{type Dict}
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
  |> build_mat
  |> count_ans(0)
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

fn count_ans(mat: Mat, acc: Int) -> Int {
  let beams =
    mat
    |> dict.filter(fn(_key, val) { val == "S" || val == "|" })
    |> dict.size

  use <- bool.guard(beams == 0, acc)

  let #(new_mat, splits) = move_beams(mat)
  count_ans(new_mat, acc + splits)
}

fn move_beams(mat: Mat) -> #(Mat, Int) {
  dict.fold(mat, #(mat, 0), fn(acc, pos, val) {
    use <- bool.guard(val != "|" && val != "S", acc)

    let #(new_mat, splits) = acc
    let new_mat = dict.insert(new_mat, pos, ".")
    let next_pos = #(pos.0 + 1, pos.1)
    case dict.get(new_mat, next_pos) {
      Error(_) -> #(new_mat, splits)
      Ok(next_val) ->
        case next_val {
          "^" -> {
            let new_mat =
              new_mat
              |> dict.insert(#(next_pos.0, next_pos.1 - 1), "|")
              |> dict.insert(#(next_pos.0, next_pos.1 + 1), "|")
            #(new_mat, splits + 1)
          }
          _ -> {
            let new_mat = dict.insert(new_mat, next_pos, "|")
            #(new_mat, splits)
          }
        }
    }
  })
}
