package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"strings"
)

type Rule struct {
	Write0 int
	Move0  int
	State0 string
	Write1 int
	Move1  int
	State1 string
}

func getLastWord(line string) string {
	line = strings.ReplaceAll(line, ".", "")
	words := strings.Split(line, " ")
	return words[len(words)-1]
}

func main() {
	lines := aoc.ReadAllLines()

	state := getLastWord(lines[0])
	iter := aoc.Atoi(strings.Split(lines[1], " ")[5])

	ruleMap := make(map[string]Rule)

	for i := 3; i < len(lines); i += 10 {
		curState := strings.Split(lines[i], " ")[2]
		curState = curState[:len(curState)-1]

		rule := Rule{}
		if getLastWord(lines[i+2]) == "1" {
			rule.Write0 = 1
		}
		if getLastWord(lines[i+3]) == "right" {
			rule.Move0 = 1
		} else {
			rule.Move0 = -1
		}
		rule.State0 = getLastWord(lines[i+4])

		if getLastWord(lines[i+6]) == "1" {
			rule.Write1 = 1
		}
		if getLastWord(lines[i+7]) == "right" {
			rule.Move1 = 1
		} else {
			rule.Move1 = -1
		}
		rule.State1 = getLastWord(lines[i+8])
		ruleMap[curState] = rule
	}

	valMap := make(map[int]int)
	pos := 0

	for range iter {
		rule := ruleMap[state]

		if valMap[pos] == 0 {
			valMap[pos] = rule.Write0
			pos += rule.Move0
			state = rule.State0
		} else {
			valMap[pos] = rule.Write1
			pos += rule.Move1
			state = rule.State1
		}
	}

	ans := 0
	for _, v := range valMap {
		ans += v
	}
	fmt.Println(ans)
}
