package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func hashBlocks(blocks []int) string {
	strArr := make([]string, len(blocks))
	for i, val := range blocks {
		strArr[i] = strconv.Itoa(val)
	}
	return strings.Join(strArr, "|")
}

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	blocks := make([]int, 0)
	for line := range strings.SplitSeq(string(input), "\t") {
		num, _ := strconv.Atoi(line)
		blocks = append(blocks, num)
	}

	vis := make(map[string]bool)
	vis[hashBlocks(blocks)] = true
	ans := 0
	for {
		ans += 1

		maxVal, maxIdx := blocks[0], 0
		for i, val := range blocks {
			if val > maxVal {
				maxVal = val
				maxIdx = i
			}
		}
		blocks[maxIdx] = 0
		for i := range maxVal {
			blocks[(maxIdx+1+i)%len(blocks)] += 1
		}

		hash := hashBlocks(blocks)
		if vis[hash] {
			break
		}
		vis[hash] = true
	}
	fmt.Println(ans)
}
