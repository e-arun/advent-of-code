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
  |> list.map(parse_line)
  |> dict.from_list
  |> get_ans
  |> int.to_string
  |> io.println
}

fn parse_line(line: String) -> #(String, List(String)) {
  let assert [key, values] = string.trim(line) |> string.split(": ")
  let values = string.split(values, " ")
  #(key, values)
}

fn get_ans(edges: Dict(String, List(String))) -> Int {
  dfs(edges, "you")
}

fn dfs(edges: Dict(String, List(String)), key: String) -> Int {
  use <- bool.guard(key == "out", 1)

  dict.get(edges, key)
  |> result.unwrap([])
  |> list.map(dfs(edges, _))
  |> int.sum
}
