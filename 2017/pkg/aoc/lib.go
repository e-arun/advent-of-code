package aoc

import (
	"bytes"
	"cmp"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func ArrMax[T cmp.Ordered](arr []T) T {
	ans := arr[0]
	for _, x := range arr {
		ans = max(ans, x)
	}
	return ans
}

func ArrMin[T cmp.Ordered](arr []T) T {
	ans := arr[0]
	for _, x := range arr {
		ans = min(ans, x)
	}
	return ans
}

func ArrStrToInt(arr []string) []int {
	ans := make([]int, len(arr))
	var err error

	for i, x := range arr {
		ans[i], err = strconv.Atoi(x)
		if err != nil {
			panic(fmt.Sprintf("Invalid integer: %v", x))
		}
	}
	return ans
}

func AbsInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func ReadAll() string {
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	input = bytes.TrimSpace(input)
	return string(input)
}

func ReadAllLines() []string {
	input := ReadAll()
	return strings.Split(input, "\n")
}

func Atoi(x string) int {
	val, err := strconv.Atoi(x)
	if err != nil {
		panic(err)
	}
	return val
}
