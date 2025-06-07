type Node = { x: number; y: number; size: number; used: number; avail: number };

const parseLine = (line: string): Node => {
	const words = line.split(/\s+/);
	const [_, xStr, yStr] = words[0].split("-");
	return {
		x: Number(xStr.slice(1)),
		y: Number(yStr.slice(1)),
		size: Number(words[1].slice(0, words[1].length - 1)),
		used: Number(words[2].slice(0, words[2].length - 1)),
		avail: Number(words[3].slice(0, words[3].length - 1)),
	};
};

const input = await Bun.stdin.text();
const nodes = input.trim().split("\n").slice(2).map(parseLine);

let ans = 0;
for (const a of nodes) {
	for (const b of nodes) {
		if (a === b) {
			continue;
		}
		if (a.used !== 0 && a.used <= b.avail) {
			ans += 1;
		}
	}
}
console.log(ans);
