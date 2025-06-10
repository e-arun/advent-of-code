package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	input, _ := io.ReadAll(os.Stdin)
	input = bytes.TrimSpace(input)

	adjMap := make(map[string][]string)
	replacer := strings.NewReplacer(" -> ", " ", "(", "", ")", "", ",", "")
	for line := range strings.SplitSeq(string(input), "\n") {
		line = replacer.Replace(line)
		words := strings.Split(line, " ")
		adjMap[words[0]] = words[2:]
	}

	nonRoot := make(map[string]bool)
	for _, children := range adjMap {
		for _, child := range children {
			nonRoot[child] = true
		}
	}

	for name := range adjMap {
		if !nonRoot[name] {
			fmt.Println(name)
			break
		}
	}
}
