import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
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
  dfs(dict.new(), edges, "svr", True, True) |> pair.second
}

type Cache =
  Dict(#(String, Bool, Bool), Int)

fn dfs(
  cache: Cache,
  edges: Dict(String, List(String)),
  key: String,
  needs_dac: Bool,
  needs_fft: Bool,
) -> #(Cache, Int) {
  let cache_val =
    dict.get(cache, #(key, needs_dac, needs_fft))
    |> result.map(fn(val) { #(cache, val) })

  use <- result_guard(cache_val)
  let found_both = case needs_dac || needs_fft {
    True -> 0
    False -> 1
  }
  use <- bool.guard(key == "out", #(cache, found_both))

  let targets = dict.get(edges, key) |> result.unwrap([])
  let #(cache, ans) =
    list.fold(targets, #(cache, 0), fn(acc, target) {
      let #(cache, ans) = acc
      let needs_dac = needs_dac && target != "dac"
      let needs_fft = needs_fft && target != "fft"
      let #(cache, val) = dfs(cache, edges, target, needs_dac, needs_fft)
      #(cache, ans + val)
    })

  let cache = dict.insert(cache, #(key, needs_dac, needs_fft), ans)
  #(cache, ans)
}

fn result_guard(res: Result(a, b), otherwise: fn() -> a) -> a {
  case res {
    Ok(val) -> val
    Error(_) -> otherwise()
  }
}
