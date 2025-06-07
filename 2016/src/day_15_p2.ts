type Disc = { positions: number; startPos: number };

const parseLine = (line: string): Disc => {
	const words = line.split(" ");
	const positions = Number(words[3]);
	const startPos = Number(words[words.length - 1].replace(".", ""));
	return { positions, startPos };
};

const input = await Bun.stdin.text();
const discs = input.trim().split("\n").map(parseLine);
discs.push({ positions: 11, startPos: 0 });

let ans = -1;
mainLoop: while (true) {
	ans += 1;

	for (const [i, disc] of discs.entries()) {
		if ((ans + i + 1 + disc.startPos) % disc.positions !== 0) {
			continue mainLoop;
		}
	}
	break;
}

console.log(ans);
