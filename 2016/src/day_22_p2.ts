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

const grid: Node[][] = [];
for (const node of nodes) {
	if (!grid[node.y]) {
		grid[node.y] = [];
	}
	grid[node.y][node.x] = node;
}

const tarNode = grid[0][grid[0].length - 1];

// The data contains a single free node and all nodes that have higher 'used'
// than it are on the same y and contiguous x value.
const freeNode = nodes.filter((x) => x.avail >= tarNode.size)[0];
const fullNodes = nodes.filter((x) => x.used > freeNode.avail);
const minBlockX = fullNodes
	.map((node) => node.x)
	.reduce((min, x) => (min > x ? x : min));

let ans = 0;
if (freeNode.x >= minBlockX) {
	ans += freeNode.x - minBlockX + 1;
}
ans += freeNode.y;
ans += tarNode.x - minBlockX;
ans += 5 * (tarNode.x - 1) + 1;
console.log(ans);
