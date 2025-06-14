package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"strings"
)

func rotate(s string) string {
	arr := make([]byte, len(s))

	n := 2
	if len(s) == 9 {
		n = 3
	}

	for r := range n {
		for c := range n {
			arr[(n-1-c)*n+r] = s[r*n+c]
		}
	}
	return string(arr)
}

func flip(s string) string {
	arr := make([]byte, len(s))

	n := 2
	if len(s) == 9 {
		n = 3
	}

	for r := range n {
		for c := range n {
			arr[r*n+n-1-c] = s[r*n+c]
		}
	}
	return string(arr)
}

type AnsKey struct {
	Mat  string
	Iter int
}

func getAns(ansKey AnsKey, patMap map[string]string, ansMap map[AnsKey]int) int {
	if ansKey.Iter == 0 {
		return strings.Count(ansKey.Mat, "#")
	}

	ans, ok := ansMap[ansKey]
	if ok {
		return ans
	}

	if len(ansKey.Mat) == 4 || len(ansKey.Mat) == 9 {
		newMat, ok := patMap[ansKey.Mat]
		if !ok {
			panic(ok)
		}
		newKey := AnsKey{newMat, ansKey.Iter - 1}
		ans = getAns(newKey, patMap, ansMap)
		ansMap[newKey] = ans
		return ans
	}

	if len(ansKey.Mat) == 16 {
		curMat := [4][4]byte{}
		for r := range 4 {
			for c := range 4 {
				curMat[r][c] = ansKey.Mat[r*4+c]
			}
		}

		newMat := [6][6]byte{}
		for r := range 2 {
			for c := range 2 {
				smallMat, ok := patMap[string([]byte{curMat[r*2][c*2], curMat[r*2][c*2+1], curMat[r*2+1][c*2], curMat[r*2+1][c*2+1]})]
				if !ok {
					panic(ok)
				}
				for r2 := range 3 {
					for c2 := range 3 {
						newMat[r*3+r2][c*3+c2] = smallMat[r2*3+c2]
					}
				}
			}
		}

		flatMat := make([]byte, 36)
		for r := range 6 {
			for c := range 6 {
				flatMat[r*6+c] = newMat[r][c]
			}
		}

		newKey := AnsKey{string(flatMat), ansKey.Iter - 1}
		ans = getAns(newKey, patMap, ansMap)
		ansMap[newKey] = ans
		return ans
	}

	if len(ansKey.Mat) != 36 {
		panic(ansKey.Mat)
	}

	curMat := [6][6]byte{}
	for r := range 6 {
		for c := range 6 {
			curMat[r][c] = ansKey.Mat[r*6+c]
		}
	}
	ans = 0
	for r := range 3 {
		for c := range 3 {
			smallMat := patMap[string([]byte{curMat[r*2][c*2], curMat[r*2][c*2+1], curMat[r*2+1][c*2], curMat[r*2+1][c*2+1]})]
			ans += getAns(AnsKey{smallMat, ansKey.Iter - 1}, patMap, ansMap)
		}
	}
	ansMap[ansKey] = ans
	return ans
}

func main() {
	lines := aoc.ReadAllLines()
	patMap := make(map[string]string)

	for _, line := range lines {
		line = strings.ReplaceAll(line, "/", "")
		arr := strings.Split(line, " => ")
		key := arr[0]
		val := arr[1]

		patMap[key] = val
		for range 3 {
			key = rotate(key)
			patMap[key] = val
			patMap[flip(key)] = val
		}
	}

	start := ".#...####"
	ans := getAns(AnsKey{start, 18}, patMap, make(map[AnsKey]int))
	fmt.Println(ans)
}
