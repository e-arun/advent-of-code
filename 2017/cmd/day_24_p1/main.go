package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"strings"
)

type Item struct {
	A int
	B int
}

func main() {
	lines := aoc.ReadAllLines()
	items := make([]Item, len(lines))

	for i, line := range lines {
		words := strings.Split(line, "/")
		items[i] = Item{aoc.Atoi(words[0]), aoc.Atoi(words[1])}
	}

	ans := 0

	var run func(score int, val int, mask int)
	run = func(score int, val int, mask int) {
		ans = max(ans, score)
		for i, item := range items {
			if mask>>i&1 == 1 {
				continue
			}
			if item.A == val {
				run(
					score+item.A+item.B,
					item.B,
					mask|(1<<i),
				)
			} else if item.B == val {
				run(
					score+item.A+item.B,
					item.A,
					mask|(1<<i),
				)
			}
		}
	}
	run(0, 0, 0)

	fmt.Println(ans)
}
