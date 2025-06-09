package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	ans := 0

	for i := range len(input) {
		nextVal := input[(i+1)%len(input)]
		if input[i] == nextVal {
			ans += int(input[i] - '0')
		}
	}

	fmt.Println(ans)
}
