package main

import (
	"aoc/pkg/aoc"
	"fmt"
	"slices"
)

type Particle struct {
	Idx int
	P   [3]int
	V   [3]int
	A   [3]int
}

func sumArr(xs [3]int) int {
	ans := 0
	for _, x := range xs {
		ans += aoc.AbsInt(x)
	}
	return ans
}

func cmpParticle(a, b Particle) int {
	valA := [3]int{sumArr(a.A), sumArr(a.V), sumArr(a.P)}
	valB := [3]int{sumArr(b.A), sumArr(b.V), sumArr(b.P)}

	for i := range 3 {
		if valA[i] < valB[i] {
			return -1
		}
		if valA[i] > valB[i] {
			return 1
		}
	}
	return 0
}

func main() {
	lines := aoc.ReadAllLines()
	particles := make([]Particle, 0)

	for i, line := range lines {
		p := Particle{
			Idx: i,
			P:   [3]int{},
			V:   [3]int{},
			A:   [3]int{},
		}
		fmt.Sscanf(
			line,
			"p=<%v,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>",
			&p.P[0], &p.P[1], &p.P[2],
			&p.V[0], &p.V[1], &p.V[2],
			&p.A[0], &p.A[1], &p.A[2],
		)
		particles = append(particles, p)
	}

	slices.SortFunc(particles, cmpParticle)
	fmt.Println(particles[0].Idx)
}
