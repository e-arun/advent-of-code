package main

import (
	"fmt"
)

func main() {
	// The input was converted into a simple go program with loops instead of jumps
	// The innermost loop was simplified after manual inspection, and the result
	// was fast enough.

	a := 0
	b := 0
	c := 0
	d := 0
	e := 0
	f := 0
	g := 0
	h := 0

	_ = a // to prevent unused compiler error
	_ = e // to prevent unused compiler error

	a = 1
	b = 57*100 + 100000
	c = b + 17000

	for {
		f = 1
		d = 2
		for {
			e = 2
			for {
				// Manually simplified
				if b%d == 0 {
					f = 0
				}
				e = b
				g = 0
				if g == 0 {
					break
				}
			}
			d += 1
			g = d - b
			if g == 0 {
				break
			}
		}
		if f == 0 {
			h += 1
		}
		g = b - c
		if g == 0 {
			break
		}
		b += 17
	}
	fmt.Println(h)
}
