const input = await Bun.stdin.text();
const lines = input.trim().split("\n");

let reg = { a: 0, b: 0, c: 0, d: 0 };
type RegKey = keyof typeof reg;
const isRegKey = (key: string): key is RegKey => {
	return key in reg;
};

const isValidTransmission = (transmission: number[]): boolean => {
	for (const [i, val] of transmission.entries()) {
		if (i % 2 !== val) {
			return false;
		}
	}
	return true;
};

let ans = 0;
while (true) {
	ans += 1;
	reg = { a: ans, b: 0, c: 0, d: 0 };
	const transmission: number[] = [];

	for (let i = 0; i < lines.length; i++) {
		const [instr, x, y] = lines[i].split(" ");

		if (instr === "cpy") {
			const val = isRegKey(x) ? reg[x] : Number(x);
			reg[y as RegKey] = val;
		} else if (instr === "inc") {
			reg[x as RegKey] += 1;
		} else if (instr === "dec") {
			reg[x as RegKey] -= 1;
		} else if (instr === "jnz") {
			const val = isRegKey(x) ? reg[x] : Number(x);
			if (val !== 0) {
				i += Number(y);
				i--;
			}
		} else if (instr === "out") {
			transmission.push(isRegKey(x) ? reg[x] : Number(x));
			if (transmission.length > 20) {
				break;
			}
		}
	}

	if (isValidTransmission(transmission)) {
		console.log(ans);
		break;
	}
}
