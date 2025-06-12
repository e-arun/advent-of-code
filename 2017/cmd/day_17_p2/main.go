package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)
	steps, _ := strconv.Atoi(string(input))
	steps += 1

	cur := 0
	ans := 0

	for i := 1; i <= 50_000_000; i++ {
		cur = (cur + steps) % i
		if cur == 0 {
			ans = i
		}
		skippable := (i - cur) / steps
		cur += skippable * steps
		i += skippable
	}

	fmt.Println(ans)
}
