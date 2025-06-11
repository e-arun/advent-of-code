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

	adjMap := make(map[int][]int)

	for line := range strings.SplitSeq(string(input), "\n") {
		line = strings.ReplaceAll(line, ",", "")
		words := strings.Split(line, " ")

		arr := make([]int, 0)
		for _, word := range words[2:] {
			n, _ := strconv.Atoi(word)
			arr = append(arr, n)
		}

		n, _ := strconv.Atoi(words[0])
		adjMap[n] = arr
	}

	groups := 0
	vis := make(map[int]bool)

	for i := range adjMap {
		if vis[i] {
			continue
		}
		groups += 1
		pending := make([]int, 0)
		pending = append(pending, i)

		for len(pending) != 0 {
			n := pending[0]
			pending = pending[1:]

			if vis[n] {
				continue
			}
			vis[n] = true
			pending = append(pending, adjMap[n]...)
		}
	}
	fmt.Println(groups)
}
