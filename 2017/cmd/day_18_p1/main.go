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
	lastSound := 0
	idx := 0

mainLoop:
	for idx >= 0 && idx < len(lines) {
		words := strings.Split(lines[idx], " ")
		switch words[0] {
		case "snd":
			lastSound = getVal(words[1], regMap)
		case "set":
			regMap[words[1]] = getVal(words[2], regMap)
		case "add":
			regMap[words[1]] += getVal(words[2], regMap)
		case "mul":
			regMap[words[1]] *= getVal(words[2], regMap)
		case "mod":
			regMap[words[1]] %= getVal(words[2], regMap)
		case "rcv":
			if getVal(words[1], regMap) != 0 {
				fmt.Println(lastSound)
				break mainLoop
			}
		case "jgz":
			if getVal(words[1], regMap) > 0 {
				idx += getVal(words[2], regMap)
				continue mainLoop
			}
		}
		idx += 1
	}

}
