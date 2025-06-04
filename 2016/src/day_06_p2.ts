const leastCommon = (arr: string[]): string => {
	const map = new Map<string, number>();
	for (const item of arr) {
		map.set(item, (map.get(item) ?? 0) + 1);
	}
	return map
		.entries()
		.toArray()
		.sort((a, b) => (a[1] < b[1] ? -1 : 1))[0][0];
};

const input = await Bun.stdin.text();
const lines = input.trim().split("\n");

const ans: string[] = [];
for (let i = 0; i < lines[0].length; i++) {
	const char = leastCommon(lines.map((x) => x[i]));
	ans.push(char);
}
console.log(ans.join(""));
