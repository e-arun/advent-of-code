package main

import (
	"aoc/pkg/aoc"
	"fmt"
)

type Particle struct {
	Idx         int
	IsDestroyed bool
	P           [3]int
	V           [3]int
	A           [3]int
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

	lastCollision := 0
	curIter := 0

	for curIter-lastCollision < 1000 {
		curIter += 1

		m := make(map[[3]int]int)

		for pi := range particles {
			p := &particles[pi]
			p.V[0] += p.A[0]
			p.V[1] += p.A[1]
			p.V[2] += p.A[2]

			p.P[0] += p.V[0]
			p.P[1] += p.V[1]
			p.P[2] += p.V[2]

			idx, ok := m[p.P]
			if ok {
				particles[idx].IsDestroyed = true
				p.IsDestroyed = true
				lastCollision = curIter
			} else {
				m[p.P] = p.Idx
			}
		}
	}

	ans := 0
	for _, p := range particles {
		if !p.IsDestroyed {
			ans += 1
		}
	}
	fmt.Println(ans)
}
