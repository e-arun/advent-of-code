import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  stdin.read_lines()
  |> yielder.to_list
  |> list.map(parse_line)
  |> list.map(get_ans)
  |> int.sum
  |> int.to_string
  |> io.println
}

type Machine {
  Machine(lights: List(Int), buttons: List(Set(Int)))
}

fn parse_line(line: String) -> Machine {
  let assert [lights, ..words] = string.trim(line) |> string.split(" ")
  let lights =
    lights
    |> string.slice(1, string.length(lights) - 2)
    |> string.to_graphemes()
    |> list.map(fn(ch) {
      case ch {
        "#" -> 1
        _ -> 0
      }
    })

  let buttons =
    list.take(words, list.length(words) - 1) |> list.map(parse_button)

  Machine(lights, buttons)
}

fn parse_button(line: String) -> Set(Int) {
  let n = string.length(line)
  string.slice(line, 1, n - 2)
  |> string.split(",")
  |> list.map(int.parse)
  |> result.values
  |> set.from_list
}

fn get_ans(m: Machine) -> Int {
  let start = list.map(m.lights, fn(_) { 0 })
  get_ans_loop(m, 0, set.new(), [start])
}

fn get_ans_loop(
  m: Machine,
  dist: Int,
  vis: Set(List(Int)),
  arr: List(List(Int)),
) -> Int {
  let #(arr, vis, is_done) =
    list.fold_until(arr, #([], vis, False), fn(acc, state) {
      let #(new_arr, vis, _) = acc

      let new_states =
        list.map(m.buttons, push_button(state, _))
        |> list.filter(fn(x) { !set.contains(vis, x) })
      let vis = set.union(vis, set.from_list(new_states))

      let solved = list.any(new_states, fn(x) { x == m.lights })
      case solved {
        True -> list.Stop(#(new_arr, vis, True))
        False -> {
          let new_arr = list.append(new_states, new_arr)
          list.Continue(#(new_arr, vis, False))
        }
      }
    })

  case is_done {
    True -> dist + 1
    False -> get_ans_loop(m, dist + 1, vis, arr)
  }
}

fn push_button(lights: List(Int), button: Set(Int)) -> List(Int) {
  list.index_map(lights, fn(x, i) {
    case set.contains(button, i) {
      True -> { x + 1 } % 2
      False -> x
    }
  })
}
