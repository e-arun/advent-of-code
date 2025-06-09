package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"slices"
	"strings"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	ans := 0

mainLoop:
	for line := range strings.SplitSeq(string(input), "\n") {
		m := make(map[string]bool)

		for word := range strings.SplitSeq(line, " ") {
			arr := []rune(word)
			slices.Sort(arr)
			word = string(arr)

			if m[word] {
				continue mainLoop
			}
			m[word] = true
		}
		ans += 1
	}
	fmt.Println(ans)
}
