package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"strconv"
	"strings"
)

func getVal(x string, regMap map[string]int) int {
	val, err := strconv.Atoi(x)
	if err != nil {
		return regMap[x]
	}
	return val
}

func main() {
	lines := aoc.ReadAllLines()
	regMap := make(map[string]int)
	idx := 0
	ans := 0

mainLoop:
	for idx >= 0 && idx < len(lines) {
		words := strings.Split(lines[idx], " ")
		switch words[0] {
		case "set":
			regMap[words[1]] = getVal(words[2], regMap)
		case "sub":
			regMap[words[1]] -= getVal(words[2], regMap)
		case "mul":
			ans += 1
			regMap[words[1]] *= getVal(words[2], regMap)
		case "jnz":
			if getVal(words[1], regMap) != 0 {
				idx += getVal(words[2], regMap)
				continue mainLoop
			}
		}
		idx += 1
	}
	fmt.Println(ans)
}
