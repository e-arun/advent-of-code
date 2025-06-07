const input = await Bun.stdin.text();
const lines = input.trim().split("\n");

const reg = { a: 0, b: 0, c: 0, d: 0 };
type RegKey = keyof typeof reg;
const isRegKey = (key: string): key is RegKey => {
	return key in reg;
};

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
	}
}

console.log(reg.a);
