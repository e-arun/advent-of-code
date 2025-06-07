const input = await Bun.stdin.text();
const lines = input
	.trim()
	.split("\n")
	.map((x) => x.split("-").map(Number));

lines.sort((a, b) => a[0] - b[0]);

let minVal = 0;
let ans = 0;

for (const [a, b] of lines) {
	if (a > minVal) {
		ans += a - minVal;
		minVal = a + 1;
	}
	if (b > minVal) {
		minVal = b + 1;
	}
}

console.log(ans);
