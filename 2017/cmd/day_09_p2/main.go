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

	for _, ch := range input {
		if isCancelled {
			isCancelled = false
			continue
		}
		if ch == '!' {
			isCancelled = true
			continue
		}
		if !isGarbage && ch == '<' {
			isGarbage = true
			continue
		}
		if isGarbage && ch == '>' {
			isGarbage = false
			continue
		}
		if isGarbage {
			ans += 1
			continue
		}
	}

	fmt.Println(ans)
}
