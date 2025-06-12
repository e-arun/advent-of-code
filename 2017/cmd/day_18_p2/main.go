package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"strconv"
	"strings"
)

type Program struct {
	RegMap map[string]int
	Idx    int
	Msgs   []int
	Sent   int
}

func getVal(x string, regMap map[string]int) int {
	val, err := strconv.Atoi(x)
	if err != nil {
		return regMap[x]
	}
	return val
}

func runTillBlock(p *Program, lines []string) []int {
	send := make([]int, 0)

mainLoop:
	for p.Idx >= 0 && p.Idx < len(lines) {
		words := strings.Split(lines[p.Idx], " ")
		switch words[0] {
		case "snd":
			p.Sent += 1
			send = append(send, getVal(words[1], p.RegMap))
		case "set":
			p.RegMap[words[1]] = getVal(words[2], p.RegMap)
		case "add":
			p.RegMap[words[1]] += getVal(words[2], p.RegMap)
		case "mul":
			p.RegMap[words[1]] *= getVal(words[2], p.RegMap)
		case "mod":
			p.RegMap[words[1]] %= getVal(words[2], p.RegMap)
		case "rcv":
			if len(p.Msgs) == 0 {
				return send
			}
			p.RegMap[words[1]] = p.Msgs[0]
			p.Msgs = p.Msgs[1:]
		case "jgz":
			if getVal(words[1], p.RegMap) > 0 {
				p.Idx += getVal(words[2], p.RegMap)
				continue mainLoop
			}
		}
		p.Idx += 1
	}

	return send
}

func main() {
	lines := aoc.ReadAllLines()

	p0 := Program{
		RegMap: make(map[string]int),
		Idx:    0,
		Msgs:   make([]int, 0),
		Sent:   0,
	}
	p1 := Program{
		RegMap: make(map[string]int),
		Idx:    0,
		Msgs:   make([]int, 0),
		Sent:   0,
	}
	p1.RegMap["p"] = 1

	for {
		deadlock := true

		if p0.Idx >= 0 && p0.Idx < len(lines) {
			send := runTillBlock(&p0, lines)
			if len(send) == 0 {
				break
			}
			p1.Msgs = append(p1.Msgs, send...)
			deadlock = false
		}
		if p1.Idx >= 0 && p1.Idx < len(lines) {
			send := runTillBlock(&p1, lines)
			if len(send) == 0 {
				break
			}
			p0.Msgs = append(p0.Msgs, send...)
			deadlock = false
		}
		if deadlock {
			break
		}
	}

	fmt.Println(p1.Sent)

}
