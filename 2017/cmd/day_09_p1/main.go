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
	isGarbage := false
	isCancelled := false
	level := 0

	for _, ch := range input {
		if isCancelled {
			isCancelled = false
			continue
		}
		if ch == '!' {
			isCancelled = true
			continue
		}
		if ch == '<' {
			isGarbage = true
			continue
		}
		if isGarbage && ch == '>' {
			isGarbage = false
			continue
		}
		if isGarbage {
			continue
		}
		if ch == '{' {
			level += 1
			ans += level
			continue
		}
		if ch == '}' {
			level -= 1
			continue
		}
	}

	fmt.Println(ans)
}
