const parseLine = (line: string) => {
	let [prefix, checksum] = line.split("[");
	checksum = checksum.slice(0, -1);

	const arr = prefix.split("-");
	const sectorId = Number(arr[arr.length - 1]);
	const name = arr.slice(0, -1).join("-");
	return { name, checksum, sectorId };
};

const getChecksum = (name: string): string => {
	const map = new Map<string, number>();
	for (const char of name) {
		if (char === "-") {
			continue;
		}
		map.set(char, (map.get(char) ?? 0) + 1);
	}

	const entries = map.entries().toArray();
	entries.sort((a, b) => {
		if (a[1] === b[1]) {
			return a[0] < b[0] ? -1 : 1;
		}
		return a[1] < b[1] ? 1 : -1;
	});
	return entries
		.slice(0, 5)
		.map((x) => x[0])
		.join("");
};

const rotateName = (name: string, n: number): string => {
	const arr: number[] = [];
	for (const char of name) {
		if (char === "-") {
			arr.push(" ".charCodeAt(0));
			continue;
		}

		let idx = char.charCodeAt(0) - "a".charCodeAt(0);
		idx = ((idx + n) % 26) + "a".charCodeAt(0);
		arr.push(idx);
	}
	return String.fromCharCode(...arr);
};

const input = await Bun.stdin.text();
const lines = input.trim().split("\n").map(parseLine);
lines
	.filter((x) => x.checksum === getChecksum(x.name))
	.map((x) => ({ ...x, name: rotateName(x.name, x.sectorId) }))
	.filter((x) => x.name === "northpole object storage")
	.map((x) => console.log(x.sectorId));
