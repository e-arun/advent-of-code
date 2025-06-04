import { batched } from "./lib";

const ROWS = 6;
const COLS = 50;

type Instr =
	| { kind: "rect"; x: number; y: number }
	| { kind: "row"; y: number; dist: number }
	| { kind: "col"; x: number; dist: number };

const parseLine = (line: string): Instr => {
	const words = line.split(" ");
	if (words[0] === "rect") {
		const [x, y] = words[1].split("x").map(Number);
		return { kind: "rect", x, y };
	}
	if (words[1] === "column") {
		const x = Number(words[2].slice(2));
		const dist = Number(words[4]);
		return { kind: "col", x, dist };
	}
	if (words[1] === "row") {
		const y = Number(words[2].slice(2));
		const dist = Number(words[4]);
		return { kind: "row", y, dist };
	}
	throw line;
};

const executeInstr = (grid: boolean[][], instr: Instr): boolean[][] => {
	const newGrid = structuredClone(grid);
	if (instr.kind === "rect") {
		for (let r = 0; r < instr.y; r++) {
			for (let c = 0; c < instr.x; c++) {
				newGrid[r][c] = true;
			}
		}
	} else if (instr.kind === "col") {
		for (let r = 0; r < ROWS; r++) {
			newGrid[(r + instr.dist) % ROWS][instr.x] = grid[r][instr.x];
		}
	} else if (instr.kind === "row") {
		for (let c = 0; c < COLS; c++) {
			newGrid[instr.y][(c + instr.dist) % COLS] = grid[instr.y][c];
		}
	}
	return newGrid;
};

let grid: boolean[][] = [];

for (let r = 0; r < ROWS; r++) {
	grid.push(new Array(COLS).fill(false));
}

const input = await Bun.stdin.text();
const instrArr = input.trim().split("\n").map(parseLine);

for (const instr of instrArr) {
	grid = executeInstr(grid, instr);
}

for (const row of grid) {
	let rowStr = row.map((x) => (x ? "#" : " ")).join("");
	rowStr = batched(rowStr, 5).join(" ");
	console.log(rowStr);
}
