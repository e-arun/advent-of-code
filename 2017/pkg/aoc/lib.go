package aoc

import (
	"cmp"
	"fmt"
	"strconv"
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
