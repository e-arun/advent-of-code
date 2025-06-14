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

	ans := [2]int{}

	var run func(score [2]int, val int, mask int)
	run = func(score [2]int, val int, mask int) {
		if score[0] > ans[0] {
			ans = score
		} else if score[0] == ans[0] && score[1] > ans[1] {
			ans = score
		}

		for i, item := range items {
			if mask>>i&1 == 1 {
				continue
			}
			if item.A == val {
				run(
					[2]int{score[0] + 1, score[1] + item.A + item.B},
					item.B,
					mask|(1<<i),
				)
			} else if item.B == val {
				run(
					[2]int{score[0] + 1, score[1] + item.A + item.B},
					item.A,
					mask|(1<<i),
				)
			}
		}
	}
	run([2]int{0, 0}, 0, 0)

	fmt.Println(ans[1])
}
