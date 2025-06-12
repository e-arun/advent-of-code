package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

func knotHash(input []byte) []byte {
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
	return dense
}

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	var arr [128][128]bool
	for r := range 128 {
		key := string(input) + "-" + strconv.Itoa(r)
		hash := knotHash([]byte(key))

		for c1, val := range hash {
			for c2 := range 8 {
				if (val>>c2)&1 == 1 {
					arr[r][c1*8+7-c2] = true
				}
			}
		}
	}

	groups := 0
	vis := make(map[[2]int]bool)
	dirs := [4][2]int{
		{0, 1},
		{0, -1},
		{1, 0},
		{-1, 0},
	}

	for r := range 128 {
		for c := range 128 {
			pos := [2]int{r, c}
			if !arr[r][c] {
				continue
			}
			if vis[pos] {
				continue
			}

			groups += 1
			pending := make([][2]int, 0)
			pending = append(pending, pos)

			for len(pending) != 0 {
				n := pending[0]
				pending = pending[1:]

				if vis[n] {
					continue
				}
				vis[n] = true

				for _, dir := range dirs {
					nr := n[0] + dir[0]
					nc := n[1] + dir[1]
					if nr < 0 || nr >= 128 || nc < 0 || nc >= 128 {
						continue
					}
					if arr[nr][nc] {
						pending = append(pending, [2]int{nr, nc})
					}
				}
			}
		}
	}
	fmt.Println(groups)
}
