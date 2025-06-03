const input = await Bun.stdin.text();
const instructions = input.trim().split("\n");

const keypad = [
	[1, 2, 3],
	[4, 5, 6],
	[7, 8, 9],
];
let curPos = { x: 1, y: 1 };
const answer: number[] = [];

for (const instruction of instructions) {
	for (const dir of instruction) {
		const oldPos = { ...curPos };

		if (dir === "U") {
			curPos.y -= 1;
		} else if (dir === "D") {
			curPos.y += 1;
		} else if (dir === "L") {
			curPos.x -= 1;
		} else if (dir === "R") {
			curPos.x += 1;
		} else {
			throw dir;
		}

		if (keypad[curPos.y]?.[curPos.x] === undefined) {
			curPos = oldPos;
		}
	}
	answer.push(keypad[curPos.y]![curPos.x]!);
}

console.log(answer.join(""));
