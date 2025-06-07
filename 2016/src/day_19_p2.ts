const input = await Bun.stdin.text().then((x) => Number(x.trim()));

let left: number[] = [];
let right: number[] = [];

if (input % 2 === 0) {
	const mid = input / 2;
	left = [...Array(mid).keys()];
	right = [...Array(mid).keys()].map((x) => x + mid);
} else {
	const mid = (input - 1) / 2;
	left = [...Array(mid + 1).keys()];
	right = [...Array(mid).keys()].map((x) => x + mid + 1);
}

while (right.length) {
	if (left.length === right.length) {
		right.shift();
	} else {
		left.pop();
	}

	const a = left.shift()!;
	right.push(a);

	const b = right.shift()!;
	left.push(b);
}

// adding 1 to convert from 0 based to 1 based indexing
console.log(left[0] + 1);
