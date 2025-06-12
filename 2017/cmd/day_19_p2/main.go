package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

func isInBounds(arr []string, r, c int) bool {
	if r < 0 || r >= len(arr) {
		return false
	}
	if c < 0 || c >= len(arr[r]) {
		return false
	}
	return true
}

func main() {
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	arr := strings.Split(string(input), "\n")
	if arr[len(arr)-1] == "" {
		arr = arr[:len(arr)-1]
	}

	r := 0
	c := strings.Index(arr[r], "|")
	dir := "down"

	ans := 0

	for {
		ans += 1
		if dir == "down" || dir == "up" {
			nr, nc := 0, 0
			if dir == "down" {
				nr, nc = r+1, c
			} else {
				nr, nc = r-1, c
			}
			if !isInBounds(arr, nr, nc) || arr[nr][nc] == ' ' {
				if arr[r][c] != '+' {
					break
				}
				if isInBounds(arr, r, c-1) && arr[r][c-1] != ' ' {
					dir = "left"
					nr, nc = r, c-1
				} else if isInBounds(arr, r, c+1) && arr[r][c+1] != ' ' {
					dir = "right"
					nr, nc = r, c+1
				} else {
					panic("can't move")
				}
			}
			r, c = nr, nc
		} else {
			nr, nc := 0, 0
			if dir == "left" {
				nr, nc = r, c-1
			} else {
				nr, nc = r, c+1
			}
			if !isInBounds(arr, nr, nc) || arr[nr][nc] == ' ' {
				if arr[r][c] != '+' {
					break
				}
				if isInBounds(arr, r-1, c) && arr[r-1][c] != ' ' {
					dir = "up"
					nr, nc = r-1, c
				} else if isInBounds(arr, r+1, c) && arr[r+1][c] != ' ' {
					dir = "down"
					nr, nc = r+1, c
				} else {
					panic("can't move")
				}
			}
			r, c = nr, nc
		}
	}
	fmt.Println(ans)
}
