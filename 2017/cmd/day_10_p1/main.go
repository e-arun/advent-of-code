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

	arr := make([]int, 0)
	for x := range strings.SplitSeq(string(input), ",") {
		n, _ := strconv.Atoi(x)
		arr = append(arr, n)
	}

	vals := make([]int, 256)
	for i := range 256 {
		vals[i] = i
	}

	pos := 0
	for i, n := range arr {
		for j := range n / 2 {
			left := (pos + j) % 256
			right := (pos + n - 1 - j) % 256
			vals[left], vals[right] = vals[right], vals[left]
		}
		pos += (n + i) % 256
	}

	fmt.Println(vals[0] * vals[1])
}
