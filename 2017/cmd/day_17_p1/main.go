package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"slices"
	"strconv"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)
	steps, _ := strconv.Atoi(string(input))

	arr := make([]int, 0)
	arr = append(arr, 0)

	cur := 0
	for i := 1; i <= 2017; i++ {
		cur = (cur + steps) % len(arr)
		new := make([]int, 0)
		new = append(new, arr[:cur+1]...)
		new = append(new, i)
		new = append(new, arr[cur+1:]...)
		cur += 1
		arr = new
	}

	idx := slices.Index(arr, 2017)
	fmt.Println(arr[idx+1])
}
