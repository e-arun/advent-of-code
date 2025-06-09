package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"io"
	"os"
	"regexp"
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
		checksum += aoc.ArrMax(intArr) - aoc.ArrMin(intArr)
	}

	fmt.Println(checksum)
}
