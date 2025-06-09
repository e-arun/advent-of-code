package main

import (
	"aoc/pkg/aoc"
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)
	val, _ := strconv.Atoi(string(input))

	n := 1
	for n*n < val {
		n += 2
	}

	stride := n - 1
	prev := (n - 2) * (n - 2)

	r, c := 0, 0
	if val <= prev+stride {
		c = stride
		r = prev + stride - val
	} else if val <= prev+2*stride {
		c = prev + 2*stride - val
		r = 0
	} else if val <= prev+3*stride {
		c = 0
		r = stride - (prev + 3*stride - val)
	} else {
		c = stride - (prev + 4*stride - val)
		r = stride
	}

	fmt.Println(val, n, r, c)
	ans := aoc.AbsInt(r-n/2) + aoc.AbsInt(c-n/2)
	fmt.Println(ans)
}
