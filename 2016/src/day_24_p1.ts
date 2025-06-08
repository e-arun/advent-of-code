import { permutations, windows } from "./lib";

const findDistMap = (
	mat: string[][],
	r: number,
	c: number,
): Map<string, number> => {
	const distMap = new Map<string, number>();

	let curArr = [[r, c]];
	let newArr: number[][] = [];
	let dist = 0;
	const vis = new Set<string>();

	const directions = [
		[0, 1],
		[0, -1],
		[1, 0],
		[-1, 0],
	];

	while (curArr.length) {
		for (const [r, c] of curArr) {
			const hash = JSON.stringify([r, c]);
			if (vis.has(hash)) {
				continue;
			}
			vis.add(hash);

			if (mat[r][c] !== "#" && mat[r][c] !== ".") {
				distMap.set(mat[r][c], dist);
			}

			for (const [dr, dc] of directions) {
				const nr = r + dr;
				const nc = c + dc;
				if (nr < 0 || nr >= mat.length) {
					continue;
				}
				if (nc < 0 || nc >= mat[0].length) {
					continue;
				}
				if (mat[nr][nc] !== "#") {
					newArr.push([nr, nc]);
				}
			}
		}
		curArr = newArr;
		newArr = [];
		dist += 1;
	}

	return distMap;
};

const input = await Bun.stdin.text();
const mat = input
	.trim()
	.split("\n")
	.map((x) => x.split(""));

const ductPos = new Map<string, number[]>();
for (let r = 0; r < mat.length; r++) {
	for (let c = 0; c < mat[0].length; c++) {
		if (mat[r][c] !== "#" && mat[r][c] !== ".") {
			ductPos.set(mat[r][c], [r, c]);
		}
	}
}

const distMap = new Map<string, Map<string, number>>();
for (const [duct, [r, c]] of ductPos.entries()) {
	distMap.set(duct, findDistMap(mat, r, c));
}

const ductArr = ductPos
	.keys()
	.toArray()
	.filter((x) => x !== "0");

let ans = Number.POSITIVE_INFINITY;

for (const perm of permutations(ductArr)) {
	perm.unshift("0");

	let curAns = 0;
	for (const [a, b] of windows(perm, 2)) {
		curAns += distMap.get(a)!.get(b)!;
	}
	if (curAns < ans) {
		ans = curAns;
	}
}

console.log(ans);
