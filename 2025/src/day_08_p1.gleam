import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/pair
import gleam/result
import gleam/string
import gleam/yielder
import stdin

pub fn main() {
  let points =
    stdin.read_lines()
    |> yielder.to_list
    |> list.map(parse_line)

  let n = list.length(points)
  let point_map = list.index_map(points, fn(x, i) { #(i, x) }) |> dict.from_list

  let dists =
    dict.keys(point_map)
    |> list.combination_pairs
    |> list.map(fn(pair) {
      let #(i, j) = pair
      let assert Ok(pi) = dict.get(point_map, i)
      let assert Ok(pj) = dict.get(point_map, j)
      let dist = get_dist(pi, pj)
      #(i, j, dist)
    })
    |> list.sort(fn(a, b) { int.compare(a.2, b.2) })

  list.take(dists, 1000)
  |> list.fold(ufds_new(n), fn(ufds, dist) {
    let #(i, j, _) = dist
    ufds_join(ufds, i, j)
  })
  |> ufds_sizes
  |> list.sort(int.compare)
  |> list.reverse
  |> list.take(3)
  |> list.fold(1, fn(acc, x) { acc * x })
  |> int.to_string
  |> io.println
}

type Point =
  #(Int, Int, Int)

fn parse_line(line: String) -> Point {
  let arr =
    line
    |> string.trim
    |> string.split(",")
    |> list.map(int.parse)
    |> result.values

  let assert [a, b, c] = arr
  #(a, b, c)
}

fn get_dist(a: Point, b: Point) -> Int {
  let x = int.absolute_value(a.0 - b.0)
  let y = int.absolute_value(a.1 - b.1)
  let z = int.absolute_value(a.2 - b.2)
  x * x + y * y + z * z
}

type UFDS =
  Dict(Int, Int)

fn ufds_new(n: Int) -> UFDS {
  list.range(0, n - 1)
  |> list.map(fn(i) { #(i, i) })
  |> dict.from_list
}

fn ufds_parent(ufds: UFDS, i: Int) -> #(UFDS, Int) {
  let assert Ok(p) = dict.get(ufds, i)
  use <- bool.guard(i == p, #(ufds, i))

  let #(ufds, p) = ufds_parent(ufds, p)
  let ufds = dict.insert(ufds, i, p)
  #(ufds, p)
}

fn ufds_join(ufds: UFDS, a: Int, b: Int) -> UFDS {
  let #(ufds, pa) = ufds_parent(ufds, a)
  let #(ufds, pb) = ufds_parent(ufds, b)
  dict.insert(ufds, pa, pb)
}

fn ufds_parents(ufds: UFDS) -> List(Int) {
  dict.keys(ufds)
  |> list.fold(#(ufds, []), fn(acc, x) {
    let #(ufds, parents) = acc
    let #(ufds, p) = ufds_parent(ufds, x)
    #(ufds, [p, ..parents])
  })
  |> pair.second
}

fn ufds_sizes(ufds: UFDS) -> List(Int) {
  ufds_parents(ufds)
  |> counter()
  |> dict.values
}

fn counter(xs: List(Int)) -> Dict(Int, Int) {
  list.fold(xs, dict.new(), fn(acc, x) {
    dict.upsert(acc, x, fn(val) { option.unwrap(val, 0) + 1 })
  })
}
