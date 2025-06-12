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

	arr := make([]byte, 16)
	for i := range 16 {
		arr[i] = 'a' + byte(i)
	}

	for instr := range strings.SplitSeq(string(input), ",") {
		if instr[0] == 's' {
			val, _ := strconv.Atoi(instr[1:])
			pos := len(arr) - val
			arr = append(arr[pos:], arr[:pos]...)
			continue
		}

		if instr[0] == 'x' {
			valArr := strings.Split(instr[1:], "/")
			valA, _ := strconv.Atoi(valArr[0])
			valB, _ := strconv.Atoi(valArr[1])
			arr[valA], arr[valB] = arr[valB], arr[valA]
			continue
		}

		valArr := strings.Split(instr[1:], "/")
		valA := bytes.Index(arr, []byte(valArr[0]))
		valB := bytes.Index(arr, []byte(valArr[1]))
		arr[valA], arr[valB] = arr[valB], arr[valA]
	}

	fmt.Println(string(arr))
}
