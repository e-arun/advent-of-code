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
  let edges = list.window_by_2(points)
  list.combination_pairs(points)
  |> list.filter(fn(pair) { is_valid(pair.0, pair.1, edges) })
  |> list.map(fn(pair) { get_size(pair.0, pair.1) })
  |> list.max(int.compare)
  |> result.unwrap(0)
}

fn get_size(a: Point, b: Point) -> Int {
  let l = int.absolute_value(a.0 - b.0) + 1
  let w = int.absolute_value(a.1 - b.1) + 1
  l * w
}

fn is_valid(a: Point, b: Point, edges: List(#(Point, Point))) -> Bool {
  !list.any(edges, fn(edge) { is_line_in_rect(edge.0, edge.1, a, b) })
}

fn is_line_in_rect(
  line_a: Point,
  line_b: Point,
  rect_a: Point,
  rect_b: Point,
) -> Bool {
  let line_left = int.min(line_a.0, line_b.0)
  let line_right = int.max(line_a.0, line_b.0)
  let line_top = int.min(line_a.1, line_b.1)
  let line_bot = int.max(line_a.1, rect_b.1)

  let rect_left = int.min(rect_a.0, rect_b.0)
  let rect_right = int.max(rect_a.0, rect_b.0)
  let rect_top = int.min(rect_a.1, rect_b.1)
  let rect_bot = int.max(rect_a.1, rect_b.1)

  line_left < rect_right
  && line_right > rect_left
  && line_top < rect_bot
  && line_bot > rect_top
}
