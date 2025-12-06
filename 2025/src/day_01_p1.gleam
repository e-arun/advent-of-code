import gleam/function
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/string
import gleam/yielder
import stdin

fn parse_input() {
  stdin.read_lines()
  |> yielder.map(string.trim)
  |> yielder.map(parse_line)
  |> yielder.to_list
}

fn parse_line(line: String) -> #(String, Int) {
  let assert Ok(dir) = string.first(line)
  let assert Ok(val) = string.drop_start(line, 1) |> int.parse
  #(dir, val)
}

pub fn main() {
  let lines = parse_input()

  list.map_fold(lines, 50, fn(pos, line) {
    let #(dir, val) = line
    let pos = case dir {
      "R" -> { pos + val } % 100
      _ -> { pos - val } % 100
    }
    #(pos, pos == 0)
  })
  |> pair.second
  |> list.count(function.identity)
  |> int.to_string
  |> io.println
}
