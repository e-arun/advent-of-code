import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  let input =
    stdin.read_lines()
    |> yielder.to_list
    |> list.map(string.trim)
    |> split_input([])
    |> list.map(list.reverse)

  let assert [regions, ..shapes] = input
  let shapes = list.reverse(shapes) |> list.map(list.drop(_, 1))

  let regions = list.map(regions, parse_region)
  let shape_areas = list.map(shapes, get_shape_areas)

  let #(min_bound, max_bound) = get_ans(shape_areas, regions)

  let msg = case min_bound == max_bound {
    True -> int.to_string(min_bound)
    False ->
      "Answer is between "
      <> int.to_string(min_bound)
      <> "-"
      <> int.to_string(max_bound)
  }
  io.println(msg)
}

fn split_input(
  lines: List(String),
  acc: List(List(String)),
) -> List(List(String)) {
  case lines {
    [] -> acc
    ["", ..rest] -> split_input(rest, [[], ..acc])
    [first, ..rest] ->
      case acc {
        [] -> split_input(rest, [[first]])
        [x, ..xs] -> split_input(rest, [list.append([first], x), ..xs])
      }
  }
}

fn get_shape_areas(shape: List(String)) -> Int {
  shape
  |> string.join("")
  |> string.to_graphemes
  |> list.count(fn(x) { x == "#" })
}

fn parse_region(region: String) -> #(Int, List(Int)) {
  let assert [area, shapes] = string.split(region, ": ")

  let assert [x, y] = string.split(area, "x")
  let assert Ok(x) = int.parse(x)
  let assert Ok(y) = int.parse(y)

  let shapes = string.split(shapes, " ") |> list.map(int.parse) |> result.values

  #(x * y, shapes)
}

// Solution inspired from comments on AoC Reddit
// https://www.reddit.com/r/adventofcode/comments/1pkje0o/2025_day_12_solutions/
//
// Simply check if -
// the shapes would fit if they were perfectly arraged (max_bound)
// the shapes would fit if they occupied the entire 3x3 tile without any overlap (min_bound)
// If min_bound and max_bound are the same, then you have your answer!
fn get_ans(shapes: List(Int), regions: List(#(Int, List(Int)))) -> #(Int, Int) {
  list.fold(regions, #(0, 0), fn(acc, region) {
    let #(min_bound, max_bound) = acc
    let #(region_area, region_shapes) = region

    let max_area = int.sum(region_shapes) * 9
    let min_area =
      list.zip(region_shapes, shapes)
      |> list.map(fn(x) { x.0 * x.1 })
      |> int.sum

    let min_bound = case region_area >= max_area {
      True -> min_bound + 1
      False -> min_bound
    }

    let max_bound = case region_area >= min_area {
      True -> max_bound + 1
      False -> max_bound
    }

    #(min_bound, max_bound)
  })
}
