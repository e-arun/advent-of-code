package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func mostFreq(arr []int) int {
	ctr := make(map[int]int)
	for _, val := range arr {
		ctr[val] += 1
	}

	freqVal := arr[0]
	freqCt := ctr[freqVal]

	for val, ct := range ctr {
		if ct > freqCt {
			freqCt = ct
			freqVal = val
		}
	}
	return freqVal
}

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	adjMap := make(map[string][]string)
	selfWeight := make(map[string]int)
	discWeight := make(map[string]int)

	replacer := strings.NewReplacer(" -> ", " ", "(", "", ")", "", ",", "")
	for line := range strings.SplitSeq(string(input), "\n") {
		line = replacer.Replace(line)
		words := strings.Split(line, " ")
		adjMap[words[0]] = words[2:]
		selfWeight[words[0]], _ = strconv.Atoi(words[1])
		if len(words[2:]) == 0 {
			discWeight[words[0]], _ = strconv.Atoi(words[1])
		}
	}

mainLoop:
	for {

	innerLoop:
		for name, children := range adjMap {
			if len(children) == 0 {
				continue
			}
			for _, child := range children {
				if discWeight[child] == 0 {
					continue innerLoop
				}
			}

			childWeights := make([]int, len(children))
			for i, child := range children {
				childWeights[i] = discWeight[child]
			}
			x := mostFreq(childWeights)
			total := 0

			for _, child := range children {
				if discWeight[child] != x {
					fmt.Println(x - discWeight[child] + selfWeight[child])
					break mainLoop
				}
				total += discWeight[child]
			}
			discWeight[name] = total + selfWeight[name]
		}
	}
}
