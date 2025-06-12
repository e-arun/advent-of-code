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

	lines := strings.Split(string(input), "\n")

	words := strings.Split(lines[0], " ")
	a, _ := strconv.Atoi(words[len(words)-1])
	words = strings.Split(lines[1], " ")
	b, _ := strconv.Atoi(words[len(words)-1])

	ans := 0
	mask := (1 << 16) - 1

	for range 5_000_000 {
		a = (a * 16807) % 2147483647
		for a&3 != 0 {
			a = (a * 16807) % 2147483647
		}

		b = (b * 48271) % 2147483647
		for b&7 != 0 {
			b = (b * 48271) % 2147483647
		}
		if (a & mask) == (b & mask) {
			ans += 1
		}
	}

	fmt.Println(ans)
}
