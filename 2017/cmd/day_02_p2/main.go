package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"io"
	"os"
	"regexp"
	"slices"
	"strings"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	inputStr := string(input)
	inputStr = strings.TrimSpace(inputStr)

	checksum := 0
	for line := range strings.SplitSeq(inputStr, "\n") {
		strArr := regexp.MustCompile(`\s+`).Split(line, -1)
		intArr := aoc.ArrStrToInt(strArr)
		slices.Sort(intArr)

	mainLoop:
		for i := 0; i < len(intArr); i++ {
			for j := i + 1; j < len(intArr); j++ {
				if intArr[j]%intArr[i] == 0 {
					checksum += intArr[j] / intArr[i]
					break mainLoop
				}
			}
		}
	}

	fmt.Println(checksum)
}
