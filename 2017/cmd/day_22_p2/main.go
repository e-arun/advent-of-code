package main

import (
	"aoc/pkg/aoc"
	"fmt"
)

type Pos struct {
	R int
	C int
}

const (
	clean    = 0
	weak     = 1
	infected = 2
	flagged  = 3
)

func main() {
	lines := aoc.ReadAllLines()

	mat := make(map[Pos]int)
	for r, line := range lines {
		for c, ch := range line {
			if ch == '#' {
				mat[Pos{r, c}] = infected
			}
		}
	}

	dirMap := [4][2]int{
		{-1, 0},
		{0, 1},
		{1, 0},
		{0, -1},
	}

	dir := 0
	p := Pos{len(lines) / 2, len(lines[0]) / 2}
	ans := 0

	for range 10_000_000 {
		if mat[p] == clean {
			dir = (dir + 3) % 4
		} else if mat[p] == weak {
			// do nothing
		} else if mat[p] == infected {
			dir = (dir + 1) % 4
		} else {
			dir = (dir + 2) % 4
		}

		mat[p] = (mat[p] + 1) % 4
		if mat[p] == infected {
			ans += 1
		}

		p.R += dirMap[dir][0]
		p.C += dirMap[dir][1]
	}
	fmt.Println(ans)
}
