package main

import (
	"aoc/pkg/aoc"
	"fmt"
)

type Pos struct {
	R int
	C int
}

func main() {
	lines := aoc.ReadAllLines()

	mat := make(map[Pos]bool)
	for r, line := range lines {
		for c, ch := range line {
			if ch == '#' {
				mat[Pos{r, c}] = true
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

	for range 10_000 {
		if mat[p] {
			dir = (dir + 1) % 4
		} else {
			dir = (dir + 3) % 4
		}
		mat[p] = !mat[p]
		if mat[p] {
			ans += 1
		}
		p.R += dirMap[dir][0]
		p.C += dirMap[dir][1]
	}
	fmt.Println(ans)
}
