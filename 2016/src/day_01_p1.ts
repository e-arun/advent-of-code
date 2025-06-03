const input = await Bun.stdin.text();
const instructions = input
	.trim()
	.split(", ")
	.map((x) => ({ turn: x[0]!, step: Number.parseInt(x.slice(1)) }));

const dirArr = ["N", "E", "S", "W"];
const curPos = { x: 0, y: 0 };
let curDir = 0;

for (const { turn, step } of instructions) {
	if (turn === "R") {
		curDir = (curDir + 1) % dirArr.length;
	} else {
		curDir = (curDir + 4 - 1) % dirArr.length;
	}
	switch (dirArr[curDir]) {
		case "N": {
			curPos.y -= step;
			break;
		}
		case "E": {
			curPos.x += step;
			break;
		}
		case "S": {
			curPos.y += step;
			break;
		}
		case "W": {
			curPos.x -= step;
			break;
		}
	}
}

console.log(Math.abs(curPos.x) + Math.abs(curPos.y));
