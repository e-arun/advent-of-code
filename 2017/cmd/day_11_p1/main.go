package main

import (
	"bytes"
	"fmt"
	"io"
	"math"
	"os"
	"strings"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	x := float64(0)
	y := float64(0)
	for dir := range strings.SplitSeq(string(input), ",") {
		switch dir {
		case "n":
			y -= 1
		case "ne":
			x += 0.5
			y -= 0.5
		case "se":
			x += 0.5
			y += 0.5
		case "s":
			y += 1
		case "sw":
			x -= 0.5
			y += 0.5
		case "nw":
			x -= 0.5
			y -= 0.5
		default:
			panic(dir)
		}
	}

	x = math.Abs(x)
	y = math.Abs(y)
	minVal := min(x, y)

	ans := 2*minVal + 2*(x-minVal) + 2*(y-minVal)
	fmt.Println(ans)
}
