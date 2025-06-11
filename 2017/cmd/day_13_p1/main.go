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

	ans := 0

	for line := range strings.SplitSeq(string(input), "\n") {
		line = strings.ReplaceAll(line, ":", "")
		words := strings.Split(line, " ")

		d, _ := strconv.Atoi(words[0])
		r, _ := strconv.Atoi(words[1])

		if d%(2*r-2) == 0 {
			ans += d * r
		}
	}
	fmt.Println(ans)
}
