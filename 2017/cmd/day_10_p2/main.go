package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"io"
	"os"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	arr := make([]int, len(input))
	for i, ch := range input {
		arr[i] = int(ch)
	}
	arr = append(arr, 17, 31, 73, 47, 23)

	vals := make([]int, 256)
	for i := range 256 {
		vals[i] = i
	}

	pos := 0
	skip := 0
	for range 64 {
		for _, n := range arr {
			for j := range n / 2 {
				left := (pos + j) % 256
				right := (pos + n - 1 - j) % 256
				vals[left], vals[right] = vals[right], vals[left]
			}
			pos += (n + skip) % 256
			skip += 1
		}
	}

	dense := make([]byte, 16)
	for i := range 16 {
		val := byte(0)
		for j := range 16 {
			val ^= byte(vals[i*16+j])
		}
		dense[i] = val
	}

	fmt.Println(hex.EncodeToString(dense))
}
