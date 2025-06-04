import * as lib from "./lib";

const parseLine = (line: string): number[] => {
	return line.trim().split(/\s+/).map(Number);
};

const isValidTriangle = (triangle: number[]): boolean => {
	const [a, b, c] = triangle.sort();
	return a + b > c && b + c > a && c + a > b;
};

const input = await Bun.stdin.text();
const triangles = input.trim().split("\n").map(parseLine);

const correctTriangles: number[][] = [];

for (const batch of lib.batched(triangles, 3)) {
	for (let i = 0; i < 3; i++) {
		correctTriangles.push([batch[0][i], batch[1][i], batch[2][i]]);
	}
}

const ans = lib.sum(correctTriangles.map(isValidTriangle).map(Number));
console.log(ans);
