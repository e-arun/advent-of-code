import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  stdin.read_lines()
  |> yielder.to_list
  |> list.map(string.trim)
  |> build_mat
  |> count_ans(dict.new(), 0)
  |> int.to_string
  |> io.println
}

type Mat =
  Dict(#(Int, Int), String)

type PathMat =
  Dict(#(Int, Int), Int)

fn build_mat(lines: List(String)) -> Mat {
  list.index_map(lines, fn(line, i) {
    string.to_graphemes(line)
    |> list.index_map(fn(char, j) { #(#(i, j), char) })
  })
  |> list.flatten
  |> dict.from_list
}

fn count_ans(mat: Mat, path_mat: PathMat, acc: Int) -> Int {
  let beams =
    mat
    |> dict.filter(fn(_key, val) { val == "S" || val == "|" })
    |> dict.size

  use <- bool.guard(beams == 0, acc)

  let #(new_mat, path_mat, paths) = move_beams(mat, path_mat)
  // print_mat(mat)
  count_ans(new_mat, path_mat, acc + paths)
}

fn move_beams(mat: Mat, path_mat: PathMat) -> #(Mat, PathMat, Int) {
  dict.fold(mat, #(mat, path_mat, 0), fn(acc, pos, val) {
    use <- bool.guard(val != "|" && val != "S", acc)

    let #(new_mat, path_mat, path_ct) = acc
    let new_mat = dict.insert(new_mat, pos, ".")
    let next_pos = #(pos.0 + 1, pos.1)
    let cur_path_ct = case val {
      "S" -> 1
      _ ->
        case dict.get(path_mat, pos) {
          Ok(x) -> x
          Error(_) -> panic as "No path count found"
        }
    }

    case dict.get(new_mat, next_pos) {
      Error(_) -> #(new_mat, path_mat, path_ct + cur_path_ct)
      Ok(next_val) ->
        case next_val {
          "^" -> {
            let new_mat =
              new_mat
              |> dict.insert(#(next_pos.0, next_pos.1 - 1), "|")
              |> dict.insert(#(next_pos.0, next_pos.1 + 1), "|")

            let new_path_mat =
              path_mat
              |> dict.upsert(#(next_pos.0, next_pos.1 - 1), fn(ct) {
                option.unwrap(ct, 0) + cur_path_ct
              })
              |> dict.upsert(#(next_pos.0, next_pos.1 + 1), fn(ct) {
                option.unwrap(ct, 0) + cur_path_ct
              })
            #(new_mat, new_path_mat, path_ct)
          }
          _ -> {
            let new_mat = dict.insert(new_mat, next_pos, "|")
            let new_path_mat =
              dict.upsert(path_mat, next_pos, fn(ct) {
                option.unwrap(ct, 0) + cur_path_ct
              })
            #(new_mat, new_path_mat, path_ct)
          }
        }
    }
  })
}
