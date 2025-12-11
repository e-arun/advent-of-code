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
  |> get_ans
  |> int.to_string
  |> io.println
}

fn parse_line(line: String) -> #(Int, Int) {
  let assert [a, b] =
    string.trim(line)
    |> string.split(",")
    |> list.map(int.parse)
    |> result.values
  #(a, b)
}

type Point =
  #(Int, Int)

fn get_ans(points: List(Point)) -> Int {
  list.combination_pairs(points)
  |> list.map(fn(pair) { get_size(pair.0, pair.1) })
  |> list.max(int.compare)
  |> result.unwrap(0)
}

fn get_size(a: Point, b: Point) -> Int {
  let l = int.absolute_value(a.0 - b.0) + 1
  let w = int.absolute_value(a.1 - b.1) + 1
  l * w
}
