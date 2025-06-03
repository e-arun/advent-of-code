const input = await Bun.stdin.text();
const instructions = input
	.trim()
	.split(", ")
	.map((x) => ({ turn: x[0]!, step: Number.parseInt(x.slice(1)) }));

const dirArr = ["N", "E", "S", "W"];
const curPos = { x: 0, y: 0 };
let curDir = 0;

const visited = new Set<string>();
visited.add(JSON.stringify(curPos));

mainLoop: for (const { turn, step } of instructions) {
	if (turn === "R") {
		curDir = (curDir + 1) % dirArr.length;
	} else {
		curDir = (curDir + 4 - 1) % dirArr.length;
	}

	let diff = { x: 0, y: 0 };

	switch (dirArr[curDir]) {
		case "N": {
			diff = { x: 0, y: -1 };
			break;
		}
		case "E": {
			diff = { x: 1, y: 0 };
			break;
		}
		case "S": {
			diff = { x: 0, y: 1 };
			break;
		}
		case "W": {
			diff = { x: -1, y: 0 };
			break;
		}
	}

	for (let i = 0; i < step; i++) {
		curPos.x += diff.x;
		curPos.y += diff.y;

		const curPosHash = JSON.stringify(curPos);
		if (visited.has(curPosHash)) {
			break mainLoop;
		}
		visited.add(curPosHash);
	}
}

console.log(Math.abs(curPos.x) + Math.abs(curPos.y));
