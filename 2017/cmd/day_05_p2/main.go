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

	jumps := make([]int, 0)
	for line := range strings.SplitSeq(string(input), "\n") {
		num, _ := strconv.Atoi(line)
		jumps = append(jumps, num)
	}

	ans := 0
	i := 0
	for i >= 0 && i < len(jumps) {
		ans += 1
		if jumps[i] >= 3 {
			jumps[i] -= 1
			i += jumps[i] + 1
		} else {
			jumps[i] += 1
			i += jumps[i] - 1
		}

	}
	fmt.Println(ans)
}
