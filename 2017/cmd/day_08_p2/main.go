package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	regMap := make(map[string]int)
	maxVal := 0

	for line := range strings.SplitSeq(string(input), "\n") {
		words := strings.Split(line, " ")

		reg := words[0]
		op := words[1]
		val, _ := strconv.Atoi(words[2])

		regCmp := words[4]
		cmpOp := words[5]
		cmpVal, _ := strconv.Atoi(words[6])

		cmpEval := false
		switch cmpOp {
		case "==":
			cmpEval = regMap[regCmp] == cmpVal
		case "!=":
			cmpEval = regMap[regCmp] != cmpVal
		case "<=":
			cmpEval = regMap[regCmp] <= cmpVal
		case ">=":
			cmpEval = regMap[regCmp] >= cmpVal
		case "<":
			cmpEval = regMap[regCmp] < cmpVal
		case ">":
			cmpEval = regMap[regCmp] > cmpVal
		default:
			panic(cmpOp)
		}
		if cmpEval {
			switch op {
			case "inc":
				regMap[reg] += val
			case "dec":
				regMap[reg] -= val
			default:
				panic(op)
			}
		}
		maxVal = max(maxVal, regMap[reg])
	}
	fmt.Println(maxVal)
}
