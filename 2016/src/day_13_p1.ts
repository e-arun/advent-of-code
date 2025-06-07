const inputStr = await Bun.stdin.text();
const input = Number(inputStr.trim());

const countBits = (x: number): number => {
	let tmp = x;
	let bits = 0;

	while (tmp) {
		bits += tmp & 1;
		tmp = tmp >> 1;
	}
	return bits;
};

const isWall = (x: number, y: number): boolean => {
	const val = x * x + 3 * x + 2 * x * y + y + y * y + input;
	return countBits(val) % 2 === 1;
};

const getMatrix = (x: number, y: number): string[][] => {
	const mat: string[][] = [];
	for (let r = 0; r < y; r++) {
		const matRow: string[] = [];
		for (let c = 0; c < x; c++) {
			matRow.push(isWall(c, r) ? "#" : ".");
		}
		mat.push(matRow);
	}
	return mat;
};

const printMatrix = (mat: string[][]) => {
	for (const row of mat) {
		console.log(row.join(""));
	}
};

const solve = (mat: string[][], tarX: number, tarY: number) => {
	let dist = 0;
	const vis = new Set<number>();

	let curArr = [[1, 1]];
	let nextArr: number[][] = [];

	while (curArr.length) {
		for (const [x, y] of curArr) {
			const hash = x * 1000 + y;
			if (vis.has(hash)) {
				continue;
			}
			vis.add(hash);

			if (x === tarX && y === tarY) {
				return dist;
			}

			for (const [dx, dy] of [
				[-1, 0],
				[1, 0],
				[0, 1],
				[0, -1],
			]) {
				const newX = x + dx;
				const newY = y + dy;

				if (mat[newY] && mat[newY][newX] === ".") {
					nextArr.push([newX, newY]);
				}
			}
		}
		curArr = nextArr;
		nextArr = [];

		dist += 1;
	}
};

const mat = getMatrix(100, 100);
console.log(solve(mat, 31, 39));
