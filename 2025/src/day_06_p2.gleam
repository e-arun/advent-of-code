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
  |> list.transpose
  |> list.chunk(fn(arr) { list.all(arr, fn(x) { x == " " }) })
  |> list.filter(fn(arr) {
    let assert [arr, ..] = arr
    !list.all(arr, fn(x) { x == " " })
  })
  |> list.map(get_ans)
  |> int.sum
  |> int.to_string
  |> io.println
}

fn parse_line(s: String) -> List(String) {
  s |> string.replace("\n", "") |> string.to_graphemes
}

fn get_ans(xs: List(List(String))) -> Int {
  let op =
    list.fold(xs, "", fn(op, x) {
      case list.last(x) {
        Ok("*") -> "*"
        Ok("+") -> "+"
        _ -> op
      }
    })

  xs
  |> list.map(list.filter(_, fn(x) { x != " " && x != "*" && x != "+" }))
  |> list.map(string.join(_, ""))
  |> get_single_ans(op, _)
}

fn get_single_ans(op, xs: List(String)) -> Int {
  let xs = list.map(xs, int.parse) |> result.values
  case op {
    "*" -> list.fold(xs, 1, fn(acc, x) { acc * x })
    _ -> list.fold(xs, 0, fn(acc, x) { acc + x })
  }
}
