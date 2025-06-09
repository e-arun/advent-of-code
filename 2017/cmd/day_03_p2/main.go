package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

type Point struct {
	X int
	Y int
}

func (p Point) add(other Point) Point {
	return Point{p.X + other.X, p.Y + other.Y}
}

func getGridVal(grid map[Point]int, p Point) int {
	val := 0
	for dx := -1; dx < 2; dx++ {
		for dy := -1; dy < 2; dy++ {
			val += grid[p.add(Point{dx, dy})]
		}
	}
	return val
}

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)
	val, _ := strconv.Atoi(string(input))

	grid := make(map[Point]int)
	cur := Point{0, 0}
	grid[cur] = 1

	n := 3
mainLoop:
	for {
		stride := n - 1
		loop := make([]Point, 0)

		cur.X += 1
		loop = append(loop, cur)

		for range stride - 1 {
			cur.Y -= 1
			loop = append(loop, cur)
		}
		for range stride {
			cur.X -= 1
			loop = append(loop, cur)
		}
		for range stride {
			cur.Y += 1
			loop = append(loop, cur)
		}
		for range stride {
			cur.X += 1
			loop = append(loop, cur)
		}

		for _, p := range loop {
			grid[p] = getGridVal(grid, p)
			if grid[p] > val {
				fmt.Println(grid[p])
				break mainLoop
			}
		}
		n += 2
	}
}
