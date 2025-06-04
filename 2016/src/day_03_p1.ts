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
const ans = lib.sum(triangles.map(isValidTriangle).map(Number));
console.log(ans);
